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
            0x0..=0xFF => BOOT_ROM[addr as usize],
            0x100..=0x7FFF => *self.rom_data.get(addr as usize).unwrap_or(&0),
            0x8000..=0xFFFF => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),
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

    pub fn ins(&self) -> Instruction {
        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        return parse(opcode, arg_1, arg_2);
    }

    pub fn step(&mut self) {
        let opcode = self.ins();

        match opcode {
            Instruction::Nop => self.registers.pc += 1,
            Instruction::JpImm16(pc) => self.registers.pc = pc,
            Instruction::LdR16Imm16(reg, value) => {
                self.registers.set_r16(reg, value);
                self.registers.pc += 3;
            }
            Instruction::XorAR8(reg) => {
                let r = self.registers.a ^ self.registers.get_r8(reg.clone());
                self.registers.f.zero = r == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                self.registers.set_r8(reg, r);
                self.registers.pc += 1
            }
            Instruction::LdR16memA(r16) => {
                let target_address = self.registers.get_r16_mem(r16);
                let value = self.registers.get_r8(R8::A);
                self.set_memory_byte(target_address, value);
                self.registers.pc += 1
            }
            _ => {
                dbg!(self);
                todo!();
            }
        };
    }

    pub fn format_instruction(&self) -> String {
        format!(
            "{:#06X}: \x1b[0;31m{}\x1b[0m",
            self.registers.pc,
            self.ins()
        )
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
