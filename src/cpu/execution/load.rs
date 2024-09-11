use crate::{
    cpu::CPU,
    instructions::{r16::R16, r16mem::R16mem, r8::R8},
    mmu::MMU,
};

impl CPU {
    pub(in crate::cpu) fn lda(&mut self, value: u8) {
        self.registers.a = value;
    }

    pub(in crate::cpu) fn lda_r16mem(&mut self, mmu: &mut MMU, reg: R16mem) {
        self.registers.a = mmu.read_byte(self.registers.get_r16_mem(reg))
    }

    pub(in crate::cpu) fn ld_r16mem(&mut self, mmu: &mut MMU, reg: R16mem, value: u8) {
        mmu.write_byte(self.registers.get_r16_mem(reg), value)
    }

    pub(in crate::cpu) fn ld_r8(&mut self, mmu: &mut MMU, reg: R8, value: u8) {
        self.set_r8_byte(mmu, reg, value);
    }

    pub(in crate::cpu) fn ld_r16(&mut self, reg: R16, value: u16) {
        self.registers.set_r16(reg, value)
    }

    pub(in crate::cpu) fn ldh_addr(&mut self, mmu: &mut MMU, offset: u8, value: u8) {
        mmu.write_byte(0xFF00 + offset as u16, value)
    }
}
