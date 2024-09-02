use std::{fmt, usize};

use crate::{
    bootrom::BOOT_ROM,
    instructions::{parse, r16::R16, r8::R8, Instruction},
    registers::Registers,
    rom::GBCHeader,
};

pub struct GameBoy {
    pub registers: Registers,
    pub rom_data: Vec<u8>,
    pub ram: Vec<u8>,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            registers: Registers::new(),
            rom_data,
            ram: vec![0; 0x8000],
        }
    }

    pub fn get_memory_byte(&self, addr: u16) -> u8 {
        match addr {
            // Boot Rom
            0x0..=0xFF => BOOT_ROM[addr as usize],

            // Cartridge Rom
            0x100..=0x7FFF => *self.rom_data.get(addr as usize).unwrap_or(&0),

            // Internal Ram
            0x8000..=0xFEFF => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),

            // LCD Y Register
            0xFF44 => 0x90, // Let's just set it during the VBLANK period for now...

            // Internal HRam
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
            0x8000..=0xFFFF => self.ram[(addr - 0x8000) as usize] = byte,
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

    pub fn ins(&self) -> Instruction {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        return parse(opcode, arg_1, arg_2);
    }

    pub fn step(&mut self) {
        println!("{}", self.format_instruction());
        let opcode = self.ins();

        match opcode {
            Instruction::Nop => self.registers.pc += 1,
            Instruction::JpImm16(pc) => self.registers.pc = pc,
            Instruction::LdR16Imm16(reg, value) => {
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
            Instruction::LdhCA => {
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
            Instruction::LdhImm8A(addr) => {
                let value = self.get_memory_byte(0xFF + addr as u16);
                self.set_r8_byte(R8::A, value);
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
            Instruction::LdImm16A(addr) => {
                self.registers.a = self.get_memory_byte(addr);
                self.registers.pc += 3;
            }
            Instruction::LdhAImm8(value) => {
                let target_address = 0xFF00 + value as u16;
                self.registers.a = self.get_memory_byte(target_address);
                self.registers.pc += 2
            }

            _ => {
                dbg!(self);
                todo!();
            }
        };
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
            "{:#06X}: \x1b[0;31m{}\x1b[0m",
            self.registers.pc,
            self.ins()
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
