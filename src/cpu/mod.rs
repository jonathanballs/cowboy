use colored::*;
use registers::Registers;

use crate::{
    debugger::enable_debug,
    instructions::{parse, r16::R16, r8::R8, Instruction},
    mmu::MMU,
};

mod execution;
mod flag_register;
mod registers;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub ime: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            ime: false,
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        let (instruction, mut length, cycles) = self.ins(mmu);

        let mut just_set_ei = false;
        match instruction {
            Instruction::Nop => (),

            // Load
            Instruction::LdAImm16mem(imm16) => self.lda(mmu.read_byte(imm16)),
            Instruction::LdAR16mem(reg) => self.lda_r16mem(mmu, reg),
            Instruction::LdImm16memA(addr) => mmu.write_byte(addr, self.registers.a),
            Instruction::LdR16Imm16(r, n) => self.ld_r16(r, n),
            Instruction::LdR16memA(r) => self.ld_r16mem(mmu, r, self.registers.a),
            Instruction::LdR8Imm8(r, n) => self.ld_r8(mmu, r, n),
            Instruction::LdR8R8(r, src) => self.ld_r8(mmu, r, self.get_r8_byte(mmu, src)),
            Instruction::LdhCmemA => self.ldh_addr(mmu, self.registers.c, self.registers.a),
            Instruction::LdhImm8memA(offset) => self.ldh_addr(mmu, offset, self.registers.a),
            Instruction::LdhAImm8mem(addr) => self.lda(mmu.read_byte(0xFF00 + addr as u16)),
            Instruction::LdhACmem => self.lda(mmu.read_byte(0xFF00 + self.registers.c as u16)),

            // Arithmetic
            Instruction::IncR8(reg) => self.inc(mmu, reg),
            Instruction::DecR8(reg) => self.dec(mmu, reg),
            Instruction::IncR16(reg) => self.inc_r16(reg),
            Instruction::DecR16(reg) => self.dec_r16(reg),
            Instruction::AdcAImm8(imm8) => self.adc(imm8),
            Instruction::AdcAR8(reg) => self.adc(self.get_r8_byte(mmu, reg)),
            Instruction::AddAImm8(b) => self.add(b),
            Instruction::AddAR8(reg) => self.add(self.get_r8_byte(mmu, reg)),
            Instruction::AddHlR16(reg) => self.add_r16(R16::HL, self.registers.get_r16(reg)),
            Instruction::CpAImm8(b) => self.cp(b),
            Instruction::CpAR8(reg) => self.cp(self.get_r8_byte(mmu, reg)),
            Instruction::SubAImm8(b) => self.sub(b),
            Instruction::SubAR8(reg) => self.sub(self.get_r8_byte(mmu, reg)),
            Instruction::SbcAR8(r) => self.sbc(self.get_r8_byte(mmu, r)),
            Instruction::SbcAImm8(n) => self.sbc(n),

            // Bitwise operations, checking and manipulation
            Instruction::AndAR8(reg) => self.and(self.get_r8_byte(mmu, reg)),
            Instruction::AndAImm8(imm8) => self.and(imm8),
            Instruction::OrAImm8(imm8) => self.or(imm8),
            Instruction::OrAR8(reg) => self.or(self.get_r8_byte(mmu, reg)),
            Instruction::XorAImm8(imm8) => self.xor(imm8),
            Instruction::XorAR8(reg) => self.xor(self.get_r8_byte(mmu, reg)),
            Instruction::SetB3R8(bit_index, reg) => self.set(mmu, reg, bit_index),
            Instruction::ResB3R8(bit_index, reg) => self.res(mmu, reg, bit_index),
            Instruction::BitB3R8(bit_index, reg) => self.bit(mmu, reg, bit_index),
            Instruction::Ccf => self.ccf(),

            // Bit rotation
            Instruction::RlR8(reg) => self.rl(mmu, reg),
            Instruction::Rla => self.rla(mmu),
            Instruction::SrlR8(reg) => self.srl(mmu, reg),
            Instruction::SlaR8(reg) => self.sla(mmu, reg),
            Instruction::Cpl => self.cpl(),
            Instruction::Rlca => self.rlca(),
            Instruction::Daa => self.daa(),
            Instruction::SwapR8(reg) => self.swap(mmu, reg),
            Instruction::Scf => self.scf(),
            Instruction::RrR8(r) => self.rr(mmu, r),

            // Jump instructions
            Instruction::JpImm16(addr) => self.jp(addr.wrapping_sub(length)),
            Instruction::JpHl => self.jp(self.registers.get_r16(R16::HL).wrapping_sub(length)),
            Instruction::JpCondImm16(cond, imm16) => self.jp_cond(imm16.wrapping_sub(length), cond),
            Instruction::JrCondImm8(cond, value) => self.jr_cond(value, cond),
            Instruction::JrImm8(value) => self.jr(value),

            // Call and return
            Instruction::CallCondImm16(cond, addr) => {
                if self.registers.f.evaluate_condition(cond) {
                    self.push(mmu, self.registers.pc + length);
                    self.registers.pc = addr - length;
                }
            }
            Instruction::CallImm16(addr) => {
                self.push(mmu, self.registers.pc + length);
                self.registers.pc = addr - length;
            }
            Instruction::RstTgt3(addr) => self.rst_tgt3(mmu, addr as u16),
            Instruction::Ret => self.ret(mmu),
            Instruction::RetCond(cond) => self.ret_cond(mmu, cond),
            Instruction::Reti => self.reti(mmu),
            Instruction::PushR16stk(reg) => self.push(mmu, self.registers.get_r16_stk(reg)),
            Instruction::PopR16stk(reg) => self.pop(mmu, reg),

            // Interrupt enable
            Instruction::Di => self.ime = false,
            Instruction::Ei => {
                if !self.ime {
                    just_set_ei = true;
                    self.ime = true;
                }
            }
            Instruction::Halt => length = 0,

            // Unhandled instructions
            _ => {
                println!(
                    "{}",
                    "Sorry cowboy but it looks like that instruction just ain't handled \nyet - \
                        get back out to the ranch and fix that dang emulator!"
                        .yellow()
                );
                enable_debug();
                dbg!(self.registers.pc);
            }
        };

        self.registers.pc += length;
        mmu.ppu.do_cycle(cycles as u32 / 4);

        // Increase DIV register
        mmu.timer.do_cycles(cycles);

        // Handle interrupts
        if self.ime && !just_set_ei {
            let return_pc = if matches!(instruction, Instruction::Halt) {
                self.registers.pc + 1
            } else {
                self.registers.pc
            };

            // vblank
            if mmu.ie & 1 > 0 && mmu.ppu.vblank_irq {
                mmu.ppu.vblank_irq = false;
                self.ime = false;

                self.push(mmu, return_pc);
                self.registers.pc = 0x40;
            }

            // timer
            if mmu.ie & 4 > 0 && mmu.timer.timer_irq {
                mmu.timer.timer_irq = false;
                self.ime = false;

                self.push(mmu, return_pc);
                self.registers.pc = 0x50;
            }

            // joypad
            if mmu.ie & 0x10 > 0 && mmu.joypad.joypad_irq {
                mmu.joypad.joypad_irq = false;
                self.ime = false;

                self.push(mmu, return_pc);
                self.registers.pc = 0x60;
            }
        }

        cycles
    }

    fn ins(&self, mmu: &MMU) -> (Instruction, u16, u8) {
        let opcode = mmu.read_byte(self.registers.pc);
        let arg_1 = mmu.read_byte(self.registers.pc + 1);
        let arg_2 = mmu.read_byte(self.registers.pc + 2);

        parse(opcode, arg_1, arg_2)
    }

    fn set_r8_byte(&mut self, mmu: &mut MMU, reg: R8, value: u8) {
        match reg {
            R8::HL => mmu.write_byte(self.registers.get_r16(R16::HL), value),
            _ => self.registers.set_r8(reg, value),
        }
    }

    fn get_r8_byte(&self, mmu: &MMU, reg: R8) -> u8 {
        match reg {
            R8::HL => mmu.read_byte(self.registers.get_r16(R16::HL)),
            _ => self.registers.get_r8(reg),
        }
    }

    pub fn set_memory_word(&mut self, memory: &mut MMU, addr: u16, word: u16) {
        let little = (word & 0xFF) as u8;
        let big = (word >> 8) as u8;
        memory.write_byte(addr, little);
        memory.write_byte(addr + 1, big)
    }

    pub fn get_memory_word(&mut self, memory: &MMU, addr: u16) -> u16 {
        let little = memory.read_byte(addr) as u16;
        let big = memory.read_byte(addr + 1) as u16;
        return (big << 8) | little;
    }
}
