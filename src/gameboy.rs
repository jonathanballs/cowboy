use std::{fmt, usize};

use crate::{
    bootrom::BOOT_ROM,
    instructions::{parse, Instruction},
    registers::Registers,
    rom::GBCHeader,
};

pub struct GameBoy {
    pub registers: Registers,
    pub rom_data: Vec<u8>,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            registers: Registers::new(),
            rom_data,
        }
    }

    pub fn mem(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0xFF => BOOT_ROM[addr as usize],
            0x100..=0x3FFF => *self.rom_data.get(addr as usize).unwrap_or(&0),
            _ => 0,
        }
    }

    pub fn ins(&self) -> Instruction {
        let opcode = self.mem(self.registers.pc);
        let arg_1 = self.mem(self.registers.pc + 1);
        let arg_2 = self.mem(self.registers.pc + 2);

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
            _ => {
                dbg!(self);
                todo!();
            }
        };
    }

    pub fn format_instruction(&self) -> String {
        format!("{:#06X}: {}", self.registers.pc, self.ins())
    }
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("registers", &self.registers)
            .field("rom", &GBCHeader::new(&self.rom_data))
            .field("instruction", &self.ins())
            .field("instruction_raw", &self.mem(self.registers.pc))
            .finish()
    }
}
