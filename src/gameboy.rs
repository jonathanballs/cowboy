mod debugger;

use colored::*;
use std::{fmt, sync::mpsc::Sender, usize};

use crate::{
    bootrom::BOOT_ROM,
    instructions::{parse, r16::R16, r8::R8, Instruction},
    ppu::PPU,
    registers::Registers,
    rom::GBCHeader,
};

pub struct GameBoy {
    pub registers: Registers,
    pub rom_data: Vec<u8>,
    pub ram: [u8; 0xFFFF],
    pub ppu: PPU,

    // interrupts
    ime: bool,
    ie: u8,
    ifr: u8,

    // timers
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,

    boot_rom_enabled: bool,
    debugger_enabled: bool,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>, tx: Sender<PPU>) -> GameBoy {
        GameBoy {
            boot_rom_enabled: true,

            registers: Registers::new(),
            rom_data,
            ram: [0x0; 0xFFFF],
            ppu: PPU::new(tx),
            debugger_enabled: false,

            ime: false,
            ifr: 0,
            ie: 0,

            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn step(&mut self) {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        let (instruction, mut bytes, cycles) = parse(opcode, arg_1, arg_2);

        //println!("{} {} {}", self.format_instruction(), bytes, cycles);
        //if self.registers.pc == 0xE9 {
        //    println!("{}", "Breakpoint hit. Entering debugger...".red());
        //    self.debugger_enabled = true;
        //}

        if self.debugger_enabled {
            self.debugger_cli()
        }

        match instruction {
            Instruction::Nop => (),
            Instruction::JpImm16(addr) => {
                self.registers.pc = addr;
                bytes = 0;
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
                self.registers.set_r8(reg, r);
            }
            Instruction::LdR16memA(r16) => {
                let target_address = self.registers.get_r16_mem(r16);
                let value = self.get_r8_byte(R8::A);
                self.set_memory_byte(target_address, value);
            }
            Instruction::BitB3R8(i, reg) => {
                let result = (self.get_r8_byte(reg) >> i) & 1;
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
                self.set_r8_byte(reg.clone(), self.get_r8_byte(reg.clone()).wrapping_add(1));
                let value = self.get_r8_byte(reg);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F;
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
                self.set_r8_byte(reg.clone(), self.get_r8_byte(reg.clone()).wrapping_sub(1));
                let value = self.get_r8_byte(reg);

                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F;
            }
            Instruction::LdR8R8(dest, src) => {
                self.set_r8_byte(dest, self.get_r8_byte(src));
            }
            Instruction::LdhImm8memA(addr) => {
                //if addr == 0x01 {
                //    self.debugger_cli();
                //}

                let target_address = 0xFF00 + addr as u16;
                self.set_memory_byte(target_address, self.get_r8_byte(R8::A));
            }
            Instruction::LdhAImm8mem(addr) => {
                let target_address = 0xFF00 + addr as u16;
                self.registers.a = self.get_memory_byte(target_address);
            }
            Instruction::LdAR16mem(reg) => {
                let addr = self.registers.get_r16_mem(reg);
                let value = self.get_memory_byte(addr);
                self.registers.set_r8(R8::A, value);
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
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_sub(value);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;
            }
            Instruction::AddAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_add(value);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;
            }
            Instruction::LdImm16memA(addr) => {
                self.set_memory_byte(addr, self.get_r8_byte(R8::A));
            }
            Instruction::OrAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value | self.registers.a;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::AndAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = value & self.registers.a;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            }
            Instruction::AndAImm8(imm8) => {
                let result = imm8 & self.registers.a;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            }

            Instruction::Di => {
                self.ime = false;
            }

            Instruction::Ei => {
                self.ime = false;
            }
            Instruction::Cpl => {
                self.registers.a = !self.registers.a;
                self.registers.f.half_carry = true;
                self.registers.f.subtract = true;
            }
            Instruction::SwapR8(reg) => {
                let register_value = self.get_r8_byte(reg.clone());
                let swapped = (register_value >> 4) + (register_value << 4);
                self.set_r8_byte(reg, swapped);

                self.registers.f.zero = swapped == 0;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            _ => {
                println!("{}", "Sorry cowboy but it looks like that instruction just ain't \nhandled yet - get back out to the ranch and fix that dang emulator".yellow());
                self.debugger_cli();
                todo!();
            }
        };

        self.registers.pc += bytes as u16;
        self.ppu.do_cycle(cycles as u32 / 4);
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
        //if addr == 0xff00 {
        //    self.debugger_cli();
        //}

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

            // Interrupt registers
            0xFF00 => 0x0,
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,

            0xFF0F => self.ifr,
            0xFFFF => self.ie,

            // Delegate OAM and I/O Registers to PPU
            0xFE00..=0xFF7F => self.ppu.get_byte(addr),

            // HRam
            0xFF80..=0xFFFE => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),

            // Else
            _ => {
                dbg!(&self);
                todo!()
            }
        }
    }

    pub fn set_memory_byte(&mut self, addr: u16, byte: u8) {
        //if addr == 0xff00 {
        //    self.debugger_cli();
        //}

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
            0xFF50 => self.boot_rom_enabled = byte != 0,
            0xFF0F => self.ifr = byte,

            // Joy pad input
            0xFF00 => (),

            // Serial Transfer. I will simply just not support this...
            0xFF01 => (),
            0xFF02 => (),

            // Interrupt registers
            0xFF04 => self.div = byte,
            0xFF05 => self.tima = byte,
            0xFF06 => self.tma = byte,
            0xFF07 => self.tac = byte,

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
