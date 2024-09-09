mod debugger;

use std::collections::HashSet;

use crate::cpu::CPU;
use crate::instructions::{parse, Instruction};
use crate::mmu::MMU;

pub struct GameBoy {
    // debugger
    pub debugger_enabled: bool,
    breakpoints: HashSet<u16>,
    memory_breakpoints: HashSet<u16>,

    // state
    pub mmu: MMU,
    pub cpu: CPU,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            debugger_enabled: false,

            breakpoints: HashSet::with_capacity(10),
            memory_breakpoints: HashSet::with_capacity(10),

            mmu: MMU::new(rom_data),
            cpu: CPU::new(),
        }
    }

    pub fn step(&mut self) {
        let _cycles = self.cpu.step(&mut self.mmu);
    }

    pub fn ins(&self) -> Instruction {
        let opcode = self.mmu.read_byte(self.cpu.registers.pc);
        let arg_1 = self.mmu.read_byte(self.cpu.registers.pc + 1);
        let arg_2 = self.mmu.read_byte(self.cpu.registers.pc + 2);

        match parse(opcode, arg_1, arg_2) {
            (ins, _, _) => ins,
        }
    }
}
