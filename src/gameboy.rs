mod debugger;

use colored::*;
use minifb::Key;
use std::collections::HashSet;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::usize;

use crate::{
    bootrom::BOOT_ROM,
    instructions::{parse, r16::R16, r8::R8, Instruction},
    ppu::PPU,
    registers::Registers,
};

pub struct GameBoy {
    key_rx: Receiver<(bool, Key)>,

    // debugger
    breakpoints: HashSet<u16>,
    memory_breakpoints: HashSet<u16>,

    pub registers: Registers,
    pub rom_data: Vec<u8>,
    pub ram: [u8; 0xFFFF],
    pub ppu: PPU,

    // interrupts
    ime: bool,
    ie: u8,

    // joypad
    joypad: u8,
    ssba: u8,
    dulr: u8,

    // timers
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,

    boot_rom_enabled: bool,
    pub debugger_enabled: bool,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>, tx: Sender<PPU>, rx: Receiver<(bool, Key)>) -> GameBoy {
        GameBoy {
            boot_rom_enabled: true,
            debugger_enabled: false,

            registers: Registers::new(),
            rom_data,
            ram: [0x0; 0xFFFF],
            ppu: PPU::new(tx),

            breakpoints: HashSet::with_capacity(10),
            memory_breakpoints: HashSet::with_capacity(10),

            joypad: 0,
            dulr: 0xF,
            ssba: 0xF,

            ime: false,
            ie: 0,

            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,

            key_rx: rx,
        }
    }

    pub fn start(&mut self) {
        let paused = Arc::new(AtomicBool::new(false));
        let p = paused.clone();

        ctrlc::set_handler(move || {
            if p.load(Ordering::SeqCst) {
                // If already paused, stop the emulator
                println!("{}", "\nSo long space cowboy".red());
                exit(-1);
            } else {
                // If running, pause the emulator
                p.store(true, Ordering::SeqCst);
                println!("Received Ctrl+C! Pausing at the end of this step...");
            }
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            if paused.load(Ordering::SeqCst) {
                self.debugger_enabled = true;
            }
            paused.store(false, Ordering::SeqCst);

            self.step();
        }
    }

    pub fn step(&mut self) {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        let (instruction, mut bytes, cycles) = parse(opcode, arg_1, arg_2);

        // Handle keyboard input
        loop {
            match self.key_rx.try_recv() {
                Ok((true, Key::Right)) => self.dulr &= !0x1,
                Ok((false, Key::Right)) => self.dulr |= 0x1,
                Ok((true, Key::Left)) => self.dulr &= !0x2,
                Ok((false, Key::Left)) => self.dulr |= 0x2,
                Ok((true, Key::Up)) => self.dulr &= !0x4,
                Ok((false, Key::Up)) => self.dulr |= 0x4,
                Ok((true, Key::Down)) => self.dulr &= !0x8,
                Ok((false, Key::Down)) => self.dulr |= 0x8,

                Ok((true, Key::S)) => self.ssba &= !0x1,
                Ok((false, Key::S)) => self.ssba |= 0x1,
                Ok((true, Key::A)) => self.ssba &= !0x2,
                Ok((false, Key::A)) => self.ssba |= 0x2,
                Ok((true, Key::Space)) => self.ssba &= !0x4,
                Ok((false, Key::Space)) => self.ssba |= 0x4,
                Ok((true, Key::Enter)) => self.ssba &= !0x8,
                Ok((false, Key::Enter)) => self.ssba |= 0x8,
                Err(_) => {
                    break;
                }
                _ => (),
            }
            //println!("{:02x} {:02x}", self.get_memory_byte(0xFF00), self.joypad);
        }

        // Enable the debugger immediately upon encountering a breakpoint
        if self.breakpoints.contains(&self.registers.pc) {
            self.debugger_enabled = true;
        }

        if self.debugger_enabled {
            self.debugger_cli();
        }

        match instruction {
            Instruction::Nop => (),
            Instruction::JpImm16(addr) => {
                self.registers.pc = addr;
                bytes = 0;
            }
            Instruction::JpHl => {
                self.registers.pc = self.registers.get_r16(R16::HL);
                bytes = 0;
            }
            Instruction::JpCondImm16(cond, imm16) => {
                if imm16 > 0xE000 && imm16 <= 0xFDFF {
                    self.debugger_cli();
                }

                if self.registers.f.evaluate_condition(cond) {
                    self.registers.pc = imm16;
                    bytes = 0;
                }
            }
            Instruction::LdR16Imm16mem(reg, value) => {
                self.registers.set_r16(reg, value);
            }
            Instruction::XorAR8(reg) => {
                let r = self.registers.a ^ self.get_r8_byte(reg.clone());

                self.registers.f.zero = r == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;

                self.registers.a = r;
            }
            Instruction::LdR16memA(r16) => {
                let target_address = self.registers.get_r16_mem(r16);
                let value = self.get_r8_byte(R8::A);
                self.set_memory_byte(target_address, value);
            }
            Instruction::BitB3R8(i, reg) => {
                let result = self.get_r8_byte(reg) & (1 << i);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
            }
            Instruction::JrCondImm8(cond, value) => {
                if self.registers.f.evaluate_condition(cond) {
                    self.relative_jump(value as i8);
                }
            }
            Instruction::JrImm8(value) => {
                self.relative_jump(value as i8);
            }
            Instruction::LdR8Imm8(reg, value) => {
                self.set_r8_byte(reg, value);
            }
            Instruction::LdhCmemA => {
                let target_address = 0xFF00 + self.registers.c as u16;
                self.set_memory_byte(target_address, self.registers.a);
            }
            Instruction::IncR8(reg) => {
                let value = self.get_r8_byte(reg.clone());
                let result = value.wrapping_add(1);
                self.set_r8_byte(reg.clone(), result);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                // Half carry will occur when the lower nibble was 0b1111
                self.registers.f.half_carry = (value & 0xF) == 0xF;
            }
            Instruction::IncR16(reg) => {
                self.registers.set_r16(
                    reg.clone(),
                    self.registers.get_r16(reg.clone()).wrapping_add(1),
                );

                let value = self.registers.get_r16(reg.clone());
                if matches!(reg, R16::HL) {
                    self.registers.f.zero = value == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = (value & 0x0F) == 0x0F;
                }
            }

            Instruction::DecR16(reg) => {
                self.registers.set_r16(
                    reg.clone(),
                    self.registers.get_r16(reg.clone()).wrapping_sub(1),
                );

                let value = self.registers.get_r16(reg.clone());
                if matches!(reg, R16::HL) {
                    self.registers.f.zero = value == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = (value & 0x0F) == 0x0F;
                }
            }

            Instruction::DecR8(reg) => {
                let value = self.get_r8_byte(reg.clone());
                let result = value.wrapping_sub(1);
                self.set_r8_byte(reg, result);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0xF) == 0x0;
            }
            Instruction::LdR8R8(dest, src) => {
                self.set_r8_byte(dest, self.get_r8_byte(src));
            }
            Instruction::LdhImm8memA(addr) => {
                let target_address = 0xFF00 + addr as u16;
                self.set_memory_byte(target_address, self.get_r8_byte(R8::A));
            }
            Instruction::LdhAImm8mem(addr) => {
                let target_address = 0xFF00 + addr as u16;
                self.registers.a = self.get_memory_byte(target_address);
            }
            Instruction::LdAR16mem(reg) => {
                let addr = self.registers.get_r16_mem(reg);

                if addr > 0xE000 && addr <= 0xFDFF {
                    self.debugger_cli();
                }

                let value = self.get_memory_byte(addr);
                self.registers.set_r8(R8::A, value);
            }
            Instruction::LdAImm16mem(imm16) => {
                self.registers.set_r8(R8::A, self.get_memory_byte(imm16));
            }
            Instruction::CallImm16(addr) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.pc + 3);
                self.registers.sp -= 2;
                self.registers.pc = addr;
                bytes = 0;
            }
            Instruction::RstTgt3(addr) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.pc + 1);
                self.registers.sp -= 2;
                self.registers.pc = addr as u16;
                bytes = 0;
            }

            Instruction::PushR16stk(reg) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.get_r16_stk(reg));
                self.registers.sp -= 2;
            }
            Instruction::RlR8(reg) => {
                let value = self.get_r8_byte(reg.clone());
                let new_value = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(reg, new_value);

                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value >> 7 == 1;
            }
            Instruction::Rla => {
                let value = self.get_r8_byte(R8::A);
                let new_value = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(R8::A, new_value);

                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value >> 7 == 1;
            }
            Instruction::SrlR8(reg) => {
                let value = self.get_r8_byte(reg.clone());
                let new_value = value >> 1;
                self.set_r8_byte(reg, new_value);

                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value & 1 == 1;
            }
            Instruction::SlaR8(reg) => {
                let value = self.get_r8_byte(R8::A);
                let new_value = (value << 1) | self.registers.f.carry as u8;
                self.set_r8_byte(reg, new_value);

                self.registers.f.carry = value >> 7 == 1;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.f.zero = new_value == 0;
            }

            Instruction::PopR16stk(reg) => {
                let value = self.get_memory_word(self.registers.sp);
                self.registers.set_r16_stk(reg, value);
                self.registers.sp += 2;
            }
            Instruction::Ret => {
                let addr = self.get_memory_word(self.registers.sp);
                self.registers.sp += 2;
                self.registers.pc = addr;
                bytes = 0;
            }
            Instruction::Reti => {
                let addr = self.get_memory_word(self.registers.sp);
                self.registers.sp += 2;
                self.registers.pc = addr;
                self.ime = true;
                bytes = 0;
            }
            Instruction::RetCond(cond) => {
                if self.registers.f.evaluate_condition(cond) {
                    let addr = self.get_memory_word(self.registers.sp);
                    self.registers.sp += 2;
                    self.registers.pc = addr;
                    bytes = 0;
                }
            }
            Instruction::CpAImm8(value) => {
                let result = self.registers.a.wrapping_sub(value);
                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;
            }
            Instruction::CpAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_sub(value);

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;
            }
            Instruction::SubAR8(reg) => {
                let a = self.registers.a;
                let b = self.get_r8_byte(reg);
                let result = a.wrapping_sub(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (a & 0xF) < (b & 0xF);
                self.registers.f.carry = a < b;

                self.registers.a = result;
            }
            Instruction::SubAImm8(b) => {
                let a = self.registers.a;
                let result = a.wrapping_sub(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (a & 0xF) < (b & 0xF);
                self.registers.f.carry = a < b;

                self.registers.a = result;
            }
            Instruction::AddAR8(reg) => {
                let a = self.registers.a;
                let b = self.get_r8_byte(reg);
                let result = a.wrapping_add(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                // Carry if a and b go over 0xFF
                self.registers.f.carry = (a as u16) + (b as u16) > 0xFF;
                // Half carry if the lower nibbles of a and b go over 0xF
                self.registers.f.half_carry = (a & 0xF) + (b & 0xF) > 0xF;

                self.registers.a = result;
            }
            Instruction::AddAImm8(b) => {
                let a = self.registers.a;
                let result = a.wrapping_add(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                // Carry if a and b go over 0xFF
                self.registers.f.carry = (a as u16) + (b as u16) > 0xFF;
                // Half carry if the lower nibbles of a and b go over 0xF
                self.registers.f.half_carry = (a & 0xF) + (b & 0xF) > 0xF;

                self.registers.a = result;
            }
            Instruction::OrAImm8(value) => {
                let result = value | self.registers.a;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::OrAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value | self.registers.a;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::XorAImm8(value) => {
                let result = value ^ self.registers.a;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::AddHlR16(reg) => {
                let a = self.registers.get_r16(R16::HL);
                let b = self.registers.get_r16(reg);
                let r = a.wrapping_add(b);

                self.registers.f.half_carry = (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF;
                self.registers.f.subtract = false;
                self.registers.f.carry = a > 0xFFFF - b;

                self.registers.set_r16(R16::HL, r);
            }

            Instruction::LdImm16memA(addr) => {
                self.set_memory_byte(addr, self.get_r8_byte(R8::A));
            }
            Instruction::AndAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value & self.registers.a;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            }
            Instruction::AndAImm8(imm8) => {
                let result = imm8 & self.registers.a;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            }
            Instruction::Di => {
                self.ime = false;
            }
            Instruction::Ei => {
                self.ime = true;
            }
            Instruction::Cpl => {
                self.registers.a = !self.registers.a;
                self.registers.f.half_carry = true;
                self.registers.f.subtract = true;
            }
            Instruction::Rlca => {
                let value = self.registers.a;
                self.registers.f.carry = value >> 7 > 0;
                self.registers.f.zero = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
            }

            Instruction::SwapR8(reg) => {
                let register_value = self.get_r8_byte(reg.clone());
                let swapped = (register_value >> 4) | (register_value << 4);
                self.set_r8_byte(reg, swapped);

                self.registers.f.zero = swapped == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::ResB3R8(bit_offset, reg) => {
                let result = self.get_r8_byte(reg.clone()) & !(1 << bit_offset);
                self.set_r8_byte(reg, result);
            }
            Instruction::SetB3R8(bit_offset, reg) => {
                let result = self.get_r8_byte(reg.clone()) | 1 << bit_offset;
                self.set_r8_byte(reg, result);
            }
            Instruction::Daa => {
                let mut correction = 0;
                let mut set_carry = false;

                if self.registers.f.half_carry
                    || (!self.registers.f.subtract && (self.registers.a & 0xf) > 9)
                {
                    correction |= 0x6;
                }

                if self.registers.f.carry || (!self.registers.f.subtract && self.registers.a > 0x99)
                {
                    correction |= 0x60;
                    set_carry = true;
                }

                if self.registers.f.subtract {
                    self.registers.a = self.registers.a.wrapping_sub(correction);
                } else {
                    self.registers.a = self.registers.a.wrapping_add(correction);
                }

                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.half_carry = false;
                self.registers.f.carry = set_carry;
            }
            Instruction::AdcAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self
                    .registers
                    .a
                    .wrapping_add(value)
                    .wrapping_add(self.registers.f.carry as u8);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = false;
            }
            _ => {
                println!("{}", "Sorry cowboy but it looks like that instruction just ain't handled \nyet - get back out to the ranch and fix that dang emulator!".yellow());
                self.debugger_cli();
                todo!();
            }
        };

        self.registers.pc += bytes as u16;
        self.ppu.do_cycle(cycles as u32 / 4);

        // Handle interrupts
        if self.ime {
            if self.get_memory_byte(0xFF0F) & 1 > 0 && self.ppu.vblank_irq {
                // Call 0x40
                self.ppu.vblank_irq = false;
                self.ime = false;

                self.set_memory_word(self.registers.sp - 2, self.registers.pc + 3);
                self.registers.sp -= 2;
                self.registers.pc = 0x40;
            }
        }
    }

    pub fn ins(&self) -> Instruction {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        match parse(opcode, arg_1, arg_2) {
            (ins, _, _) => ins,
        }
    }

    pub fn get_memory_byte(&self, addr: u16) -> u8 {
        // Reference: https://gbdev.io/pandocs/Memory_Map.html
        match addr {
            // Boot Rom
            0x0..=0xFF => {
                if self.boot_rom_enabled {
                    BOOT_ROM[addr as usize]
                } else {
                    *self.rom_data.get(addr as usize).unwrap_or(&0)
                }
            }

            // Cartridge Rom
            0x100..=0x7FFF => *self.rom_data.get(addr as usize).unwrap_or(&0),

            // VRAM
            0x8000..=0x9FFF => self.ppu.get_byte(addr),

            // External RAM
            0xA000..=0xBFFF => {
                todo!()
            }

            // Work RAM
            0xC000..=0xDFFF => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),

            // Echo RAM
            // In theory this maps to 0xC000..=0xDDFF but since it's not really used in practice
            // it's probably better to treat this as a canary which throws when something's gone
            // wrong!
            0xE000..=0xFDFF => unreachable!(),

            // Joy pad
            0xFF00 => {
                if self.debugger_enabled {
                    println!("{:#x} {:#x}", self.joypad, (self.joypad >> 4) & 0x3);
                }

                return match (self.joypad >> 4) & 0x3 {
                    // Return both
                    0x0 => 0xC0 | (self.dulr & self.ssba),

                    // Return select
                    0x1 => 0xD0 | self.ssba,

                    // Return dpad
                    0x2 => 0xE0 | self.dulr,

                    // Return neither
                    0x3 => 0xFF,

                    _ => unreachable!(),
                };
            }

            // Interrupt registers
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,

            0xFF0F => self.ppu.vblank_irq as u8,
            0xFF50 => {
                if self.boot_rom_enabled {
                    0x0
                } else {
                    0x1
                }
            }
            0xFFFF => self.ie,

            // Delegate OAM and I/O Registers to PPU
            0xFE00..=0xFF7F => self.ppu.get_byte(addr),

            // HRam
            0xFF80..=0xFFFE => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),
        }
    }

    pub fn set_memory_byte(&mut self, addr: u16, byte: u8) {
        if self.memory_breakpoints.contains(&addr) {
            println!(
                "{}",
                "Hit memory breakpoint... Dropping into debugger".red()
            );
            self.debugger_cli();
        }

        match addr {
            // ROM bank - ignore
            0x2000 => (),

            0x0..=0x7FFF => {
                self.debugger_cli();
                todo!();
            }

            // VRAM
            0x8000..=0x9FFF => self.ppu.set_byte(addr, byte),

            // External RAM
            0xA000..=0xBFFF => todo!(),

            // Work RAM
            0x8000..=0xDFFF => self.ram[addr as usize - 0x8000] = byte,

            // Echo RAM
            0xE000..=0xFDFF => unreachable!(),

            // Enable/disable boot rom
            0xFF50 => self.boot_rom_enabled = byte == 0,
            0xFF0F => {
                self.ppu.vblank_irq = byte & 0x1 > 0;
            }

            // Joy pad input
            0xFF00 => self.joypad = byte,

            // Serial Transfer. I will simply just not support this...
            0xFF01 => (),
            0xFF02 => (),

            // Interrupt registers
            0xFF04 => self.div = byte,
            0xFF05 => self.tima = byte,
            0xFF06 => self.tma = byte,
            0xFF07 => self.tac = byte,

            // DMA transfer from rom
            0xFF46 => {
                let source_addr = (byte as u16) << 8;
                for offset in 0x0..=0x9F {
                    self.set_memory_byte(
                        0xFE00 + offset,
                        self.get_memory_byte(source_addr + offset),
                    )
                }
            }

            // Not usable. Ignore writes...
            0xFEA0..=0xFEFF => (),

            // Delegate OAM and I/O Registers to PPU
            0xFE00..=0xFF7F => {
                //if addr == 0xFF7F {
                //    self.debugger_cli();
                //}
                self.ppu.set_byte(addr, byte)
            }

            // HRam
            0xFF80..=0xFFFE => self.ram[(addr - 0x8000) as usize] = byte,

            0xFFFF => self.ie = byte,
            //_ => {
            //    dbg!(addr);
            //    dbg!(byte);
            //    todo!()
            //}
        }
    }

    pub fn set_memory_word(&mut self, addr: u16, word: u16) {
        let little = (word & 0xFF) as u8;
        let big = (word >> 8) as u8;
        self.set_memory_byte(addr, little);
        self.set_memory_byte(addr + 1, big)
    }

    pub fn get_memory_word(&mut self, addr: u16) -> u16 {
        let little = self.get_memory_byte(addr) as u16;
        let big = self.get_memory_byte(addr + 1) as u16;
        return (big << 8) | little;
    }

    fn relative_jump(&mut self, distance: i8) {
        let _ = if distance >= 0 {
            self.registers.pc = self.registers.pc.wrapping_add(distance as u16)
        } else {
            self.registers.pc = self.registers.pc.wrapping_sub(distance.abs() as u16)
        };
    }

    fn get_r8_byte(&self, reg: R8) -> u8 {
        match reg {
            R8::HL => self.get_memory_byte(self.registers.get_r16(R16::HL)),
            _ => self.registers.get_r8(reg),
        }
    }

    fn set_r8_byte(&mut self, reg: R8, value: u8) {
        match reg {
            R8::HL => self.set_memory_byte(self.registers.get_r16(R16::HL), value),
            _ => self.registers.set_r8(reg, value),
        }
    }
}
