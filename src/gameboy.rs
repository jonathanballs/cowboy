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

    boot_rom_enabled: bool,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>, tx: Sender<PPU>) -> GameBoy {
        GameBoy {
            boot_rom_enabled: true,

            registers: Registers::new(),
            rom_data,
            ram: [0x0; 0xFFFF],
            ppu: PPU::new(tx),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.ins();
        self.ppu.do_cycle(3);

        if self.registers.pc == 0xE9 {
            println!("{}", self.format_instruction());
            dbg!(&self);
            dbg!(self.registers.get_r16(R16::HL));
            dbg!(self.registers.get_r16(R16::DE));

            //for i in 0..128 {
            //    let mem_start = i * 16;
            //    println!("{} {:x?}", i, &self.ram[mem_start..mem_start + 16]);
            //}
            //panic!("sorry");
            //sleep_ms(1000);
        }

        match opcode {
            Instruction::Nop => self.registers.pc += 1,
            Instruction::JpImm16(pc) => self.registers.pc = pc,
            Instruction::LdR16Imm16mem(reg, value) => {
                self.registers.set_r16(reg, value);
                self.registers.pc += 3;
            }
            Instruction::XorAR8(reg) => {
                let r = self.registers.a ^ self.get_r8_byte(reg.clone());
                self.registers.f.zero = r == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.set_r8(reg, r);
                self.registers.pc += 1
            }
            Instruction::LdR16memA(r16) => {
                let target_address = self.registers.get_r16_mem(r16);
                let value = self.get_r8_byte(R8::A);
                self.set_memory_byte(target_address, value);
                self.registers.pc += 1
            }
            Instruction::BitB3R8(i, reg) => {
                let result = (self.get_r8_byte(reg) >> i) & 1;
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.pc += 2
            }
            Instruction::JrCondImm8(cond, value) => {
                if self.registers.f.evaluate_condition(cond) {
                    self.relative_jump(value as i8)
                }
                self.registers.pc += 2
            }
            Instruction::JrImm8(value) => {
                self.relative_jump(value as i8);
                self.registers.pc += 2
            }
            Instruction::LdR8Imm8(reg, value) => {
                self.registers.set_r8(reg, value);
                self.registers.pc += 2
            }
            Instruction::LdhCmemA => {
                let target_address = 0xFF00 + self.registers.c as u16;
                self.set_memory_byte(target_address, self.registers.a);
                self.registers.pc += 1
            }
            Instruction::IncR8(reg) => {
                self.set_r8_byte(reg.clone(), self.get_r8_byte(reg.clone()).wrapping_add(1));
                let value = self.get_r8_byte(reg);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F;

                self.registers.pc += 1
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

                self.registers.pc += 1
            }
            Instruction::DecR8(reg) => {
                self.set_r8_byte(reg.clone(), self.get_r8_byte(reg.clone()).wrapping_sub(1));
                let value = self.get_r8_byte(reg);

                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F;

                self.registers.pc += 1
            }
            Instruction::LdR8R8(dest, src) => {
                self.set_r8_byte(dest, self.get_r8_byte(src));
                self.registers.pc += 1
            }
            Instruction::LdhImm8memA(addr) => {
                let target_address = 0xFF00 + addr as u16;
                self.set_memory_byte(target_address, self.get_r8_byte(R8::A));

                self.registers.pc += 2
            }
            Instruction::LdhAImm8mem(addr) => {
                let target_address = 0xFF00 + addr as u16;
                self.registers.a = self.get_memory_byte(target_address);
                self.registers.pc += 2
            }
            Instruction::LdAR16mem(reg) => {
                let addr = self.registers.get_r16_mem(reg);
                let value = self.get_memory_byte(addr);
                self.registers.set_r8(R8::A, value);
                self.registers.pc += 1
            }
            Instruction::CallImm16(addr) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.pc + 3);
                self.registers.sp -= 2;
                self.registers.pc = addr
            }
            Instruction::PushR16stk(reg) => {
                self.set_memory_word(self.registers.sp - 2, self.registers.get_r16_stk(reg));
                self.registers.sp -= 2;
                self.registers.pc += 1;
            }
            Instruction::RlR8(reg) => {
                let value = self.get_r8_byte(reg.clone());
                let new_value = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(reg, new_value);

                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value >> 7 == 1;

                self.registers.pc += 2;
            }
            Instruction::Rla => {
                let value = self.get_r8_byte(R8::A);
                let new_value = (value << 1) | self.registers.f.carry as u8;

                self.set_r8_byte(R8::A, new_value);

                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = value >> 7 == 1;

                self.registers.pc += 1;
            }
            Instruction::PopR16stk(reg) => {
                let value = self.get_memory_word(self.registers.sp);
                self.registers.set_r16_stk(reg, value);
                self.registers.sp += 2;
                self.registers.pc += 1;
            }
            Instruction::Ret => {
                let addr = self.get_memory_word(self.registers.sp);
                self.registers.sp += 2;
                self.registers.pc = addr;
            }
            Instruction::CpAImm8(value) => {
                let result = self.registers.a.wrapping_sub(value);
                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;

                self.registers.pc += 2;
            }
            Instruction::CpAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_sub(value);

                self.registers.f.zero = result == 0;
                dbg!(value);
                dbg!(result);
                dbg!(result == 0);
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;

                self.registers.pc += 1;
            }
            Instruction::SubAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_sub(value);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;

                self.registers.pc += 1;
            }
            Instruction::AddAR8(reg) => {
                let value = self.get_r8_byte(reg);
                let result = self.registers.a.wrapping_add(value);
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.carry = value > self.registers.a;
                self.registers.f.half_carry = (value & 0x0F) == 0x0F; // Hmmmm...
                self.registers.f.subtract = true;

                self.registers.pc += 1;
            }
            Instruction::LdImm16memA(addr) => {
                self.set_memory_byte(addr, self.get_r8_byte(R8::A));
                self.registers.pc += 3;
            }

            _ => {
                println!("{}", "Sorry cowboy but it looks like that instruction just ain't \nhandled yet - get back out to the ranch and fix that dang emulator".yellow());
                println!("{}", self.format_instruction());
                todo!();
                //self.registers.pc += 1;
            }
        };
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
        match addr {
            0x0..=0x7FFF => todo!(),

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

            // Delegate OAM and I/O Registers to PPU
            0xFE00..=0xFF7F => self.ppu.set_byte(addr, byte),

            // HRam
            0xFF80..=0xFFFE => self.ram[(addr - 0x8000) as usize] = byte,

            _ => {
                dbg!(addr);
                dbg!(byte);
                todo!()
            }
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

    pub fn format_instruction(&self) -> String {
        format!(
            "{:#06X} {:#04X}: {} ({:X?})",
            self.registers.pc,
            self.get_memory_byte(self.registers.pc),
            self.ins(),
            self.ins(),
        )
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

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("registers", &self.registers)
            .field("rom", &GBCHeader::new(&self.rom_data))
            .field("instruction", &self.ins())
            .field("instruction_raw", &self.get_memory_byte(self.registers.pc))
            .finish()
    }
}
