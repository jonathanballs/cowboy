mod debugger;

use std::collections::HashSet;

use crate::cpu::CPU;
use crate::instructions::{parse, Instruction};
use crate::mmu::MMU;
use std::collections::VecDeque;

pub struct GameBoy {
    // debugger
    breakpoints: HashSet<u16>,
    memory_breakpoints: HashSet<u16>,
    instruction_history: VecDeque<(u16, Instruction)>,

    // state
    pub mmu: MMU,
    pub cpu: CPU,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            mmu: MMU::new(rom_data),
            cpu: CPU::new(),

            breakpoints: HashSet::with_capacity(10),
            memory_breakpoints: HashSet::with_capacity(10),
            instruction_history: VecDeque::with_capacity(10000),
        }
    }

    pub fn step(&mut self) {
        if self.breakpoints.contains(&self.cpu.registers.pc) {
            self.debugger_cli();
        }

        if self.instruction_history.len() == self.instruction_history.capacity() {
            self.instruction_history.pop_front();
        }
        self.instruction_history
            .push_back((self.cpu.registers.pc, self.ins()));

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
