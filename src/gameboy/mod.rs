mod debugger;
mod timer;

use colored::*;
use std::collections::HashSet;
use timer::Timer;

use crate::mmu::MMU;
use crate::{
    instructions::{parse, r16::R16, r8::R8, Instruction},
    registers::Registers,
};

pub struct GameBoy {
    // debugger
    pub debugger_enabled: bool,
    breakpoints: HashSet<u16>,
    memory_breakpoints: HashSet<u16>,

    // state
    pub registers: Registers,
    pub mmu: MMU,

    // interrupts
    ime: bool,
    ie: u8,

    // timers
    timer: Timer,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            debugger_enabled: false,

            registers: Registers::new(),

            breakpoints: HashSet::with_capacity(10),
            memory_breakpoints: HashSet::with_capacity(10),

            mmu: MMU::new(rom_data),

            ime: false,
            ie: 0,

            timer: Timer::new(),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        let mut just_set_ei = false;

        let (instruction, mut bytes, cycles) = parse(opcode, arg_1, arg_2);
        // Enable the debugger immediately upon encountering a breakpoint
        if self.breakpoints.contains(&self.registers.pc) {
            self.debugger_enabled = true;
        }

        if self.debugger_enabled {
            self.debugger_cli();
        }

        match instruction {
            Instruction::Nop => (),

            // LOAD instructions
            Instruction::LdImm16memA(addr) => {
                self.set_memory_byte(addr, self.get_r8_byte(R8::A));
            }
            Instruction::LdR16Imm16mem(reg, value) => {
                self.registers.set_r16(reg, value);
            }
            Instruction::LdR16memA(r16) => {
                let addr = self.registers.get_r16_mem(r16);
                self.set_memory_byte(addr, self.registers.a);
            }
            Instruction::LdR8Imm8(reg, value) => {
                self.set_r8_byte(reg, value);
            }
            Instruction::LdhCmemA => {
                self.set_memory_byte(0xFF00 + self.registers.c as u16, self.registers.a);
            }
            Instruction::LdR8R8(dest, src) => {
                self.set_r8_byte(dest, self.get_r8_byte(src));
            }
            Instruction::LdhImm8memA(addr) => {
                self.set_memory_byte(0xFF00 + addr as u16, self.get_r8_byte(R8::A));
            }
            Instruction::LdhAImm8mem(addr) => {
                self.registers.a = self.get_memory_byte(0xFF00 + addr as u16);
            }
            Instruction::LdAR16mem(reg) => {
                let addr = self.registers.get_r16_mem(reg);
                self.registers.a = self.get_memory_byte(addr);
            }
            Instruction::LdAImm16mem(imm16) => {
                self.registers.a = self.get_memory_byte(imm16);
            }

            // Jump instructions
            Instruction::JpImm16(addr) => self.registers.pc = addr.wrapping_sub(3),
            Instruction::JpHl => {
                self.registers.pc = self.registers.get_r16(R16::HL).wrapping_sub(1);
            }
            Instruction::JpCondImm16(cond, imm16) => {
                if self.registers.f.evaluate_condition(cond) {
                    self.registers.pc = imm16.wrapping_sub(3);
                }
            }
            Instruction::JrCondImm8(cond, value) => {
                if self.registers.f.evaluate_condition(cond) {
                    let offset = (value as i8) as i16;
                    self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
                }
            }
            Instruction::JrImm8(value) => {
                let offset = (value as i8) as i16;
                self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
            }

            // Increment/Decrement
            Instruction::IncR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value.wrapping_add(1);
                self.set_r8_byte(reg, result);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                // Half carry will occur when the lower nibble was 0b1111
                self.registers.f.half_carry = (value & 0xF) == 0xF;
            }
            Instruction::DecR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value.wrapping_sub(1);
                self.set_r8_byte(reg, result);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                // Half carry will occur when the lower nibble was 0b0000
                self.registers.f.half_carry = (value & 0xF) == 0x0;
            }
            Instruction::IncR16(reg) => {
                let value = self.registers.get_r16(reg);
                let result = value.wrapping_add(1);
                self.registers.set_r16(reg, result);
            }
            Instruction::DecR16(reg) => {
                let value = self.registers.get_r16(reg);
                let result = value.wrapping_sub(1);
                self.registers.set_r16(reg, result);
            }

            // Bitwise operations
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
            Instruction::XorAR8(reg) => {
                let result = self.registers.a ^ self.get_r8_byte(reg);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            }

            // Bit manipulation and checking
            Instruction::SetB3R8(bit_offset, reg) => {
                let result = self.get_r8_byte(reg) | 1 << bit_offset;
                self.set_r8_byte(reg, result);
            }
            Instruction::ResB3R8(bit_offset, reg) => {
                let result = self.get_r8_byte(reg) & !(1 << bit_offset);
                self.set_r8_byte(reg, result);
            }
            Instruction::BitB3R8(i, reg) => {
                let result = self.get_r8_byte(reg) & (1 << i);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
            }

            // Call and return
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
            Instruction::Ret => {
                self.registers.pc = self.get_memory_word(self.registers.sp);
                self.registers.sp += 2;
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
            Instruction::Reti => {
                self.registers.pc = self.get_memory_word(self.registers.sp);
                self.registers.sp += 2;
                self.ime = true;
                bytes = 0;
            }

            // Push and pop
            Instruction::PushR16stk(reg) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.get_r16_stk(reg));
                self.registers.sp -= 2;
            }
            Instruction::PopR16stk(reg) => {
                let value = self.get_memory_word(self.registers.sp);
                self.registers.set_r16_stk(reg, value);
                self.registers.sp += 2;
            }

            // Maths instructions
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
            Instruction::AddHlR16(reg) => {
                let a = self.registers.get_r16(R16::HL);
                let b = self.registers.get_r16(reg);
                let result = a.wrapping_add(b);

                self.registers.f.half_carry = (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF;
                self.registers.f.subtract = false;
                self.registers.f.carry = result < a;

                self.registers.set_r16(R16::HL, result);
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
            Instruction::CpAImm8(b) => {
                let a = self.registers.a;
                let result = a.wrapping_sub(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (a & 0xF) < (b & 0xF);
                self.registers.f.carry = a < b;
            }
            Instruction::CpAR8(reg) => {
                let a = self.registers.a;
                let b = self.get_r8_byte(reg);
                let result = a.wrapping_sub(b);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (a & 0xF) < (b & 0xF);
                self.registers.f.carry = a < b;
            }

            // Bit rotation
            Instruction::RlR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(reg, result);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value >> 7 == 1;
            }
            Instruction::Rla => {
                let value = self.get_r8_byte(R8::A);
                let result = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(R8::A, result);

                // zero register is always false for RLA
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x80) != 0;
            }

            Instruction::SrlR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value >> 1;
                self.set_r8_byte(reg, result);

                self.registers.f.zero = result == 0;
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

            // Interrupt enable
            Instruction::Di => {
                self.ime = false;
            }
            Instruction::Ei => {
                if !self.ime {
                    just_set_ei = true;
                    self.ime = true;
                }
            }
            _ => {
                println!("{}", "Sorry cowboy but it looks like that instruction just ain't handled \nyet - get back out to the ranch and fix that dang emulator!".yellow());
                self.debugger_cli();
                todo!();
            }
        };

        self.registers.pc += bytes as u16;
        self.mmu.ppu.do_cycle(cycles as u32 / 4);

        // Increase DIV register
        self.timer.do_cycles(cycles);
        // Handle interrupts
        if self.ime && !just_set_ei {
            if self.get_memory_byte(0xFF0F) & 1 > 0 && self.mmu.ppu.vblank_irq {
                // Call 0x40
                self.mmu.ppu.vblank_irq = false;
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
            0x0..=0x7FFF => self.mmu.read_byte(addr),

            // VRAM
            0x8000..=0x9FFF => self.mmu.read_byte(addr),

            // External RAM
            0xA000..=0xBFFF => {
                println!("tried to read exteranl ram");
                dbg!(&self.registers);
                dbg!(addr);
                todo!()
            }

            // Work RAM
            0xC000..=0xDFFF => self.mmu.read_byte(addr),

            // Echo RAM
            // In theory this maps to 0xC000..=0xDDFF but since it's not really used in practice
            // it's probably better to treat this as a canary which throws when something's gone
            // wrong!
            0xE000..=0xFDFF => {
                println!("tried to read echo ram");
                dbg!(&self.registers);
                dbg!(addr);
                unreachable!()
            }

            // Joy pad
            0xFF00 => self.mmu.read_byte(addr),

            // Interrupt registers
            0xFF04..=0xFF07 => self.timer.read_byte(addr),

            0xFFFF => self.ie,

            // Delegate OAM and I/O Registers to PPU
            0xFE00..=0xFF7F => self.mmu.read_byte(addr),
            0xFF80..=0xFFFE => self.mmu.read_byte(addr),
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
                dbg!(addr);
                println!("tried to write to rom...");
                self.debugger_cli();
                todo!();
            }

            // VRAM
            0x8000..=0x9FFF => self.mmu.write_byte(addr, byte),

            // External RAM
            0xA000..=0xBFFF => {
                self.debugger_cli();
                todo!()
            }

            // Work RAM
            0xC000..=0xDFFF => self.mmu.write_byte(addr, byte),

            // Echo RAM
            0xE000..=0xFDFF => unreachable!(),

            // Enable/disable boot rom
            0xFF50 => self.mmu.write_byte(addr, byte),
            0xFF0F => self.mmu.write_byte(addr, byte),

            // Joy pad input
            0xFF00 => self.mmu.write_byte(addr, byte),

            // Serial Transfer. I will simply just not support this...
            0xFF01 => (),
            0xFF02 => (),

            // Div register
            0xFF04..=0xFF07 => self.timer.write_byte(addr, byte),

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
            0xFE00..=0xFF7F => self.mmu.write_byte(addr, byte),

            // HRam
            0xFF80..=0xFFFE => self.mmu.write_byte(addr, byte),

            0xFFFF => self.ie = byte,
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
