use std::fmt;

use crate::{
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
            0x0..=0x3FFF => *self.rom_data.get(addr as usize).unwrap_or(&0),
            _ => 0,
        }
    }

    fn ins(&self) -> Instruction {
        let opcode = self.mem(self.registers.pc);
        let arg_1 = self.mem(self.registers.pc + 1);
        let arg_2 = self.mem(self.registers.pc + 2);

        return parse(opcode, arg_1, arg_2);
    }

    pub fn step(&mut self) {
        let opcode = self.ins();

        if matches!(opcode, Instruction::Nop) {
            self.registers.pc += 1;
            return;
        }
    }
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("registers", &self.registers)
            .field("rom", &GBCHeader::new(&self.rom_data))
            .field("instruction", &self.ins().to_string())
            .finish()
    }
}
