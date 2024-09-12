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

    pub(in crate::cpu) fn ld_hl_sp(&mut self, e8: u8) {
        let sp = self.registers.sp;
        let offset = e8 as i8 as i16 as u16; // Convert i8 to u16 via i16 to preserve sign
        let result = sp.wrapping_add(offset);

        // Set flags based on the addition of the lower bytes
        let half_carry = (sp & 0xF) + (offset & 0xF) > 0xF;
        let carry = (sp & 0xFF) + (offset & 0xFF) > 0xFF;

        self.registers.set_r16(R16::HL, result);

        // Set flags
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = half_carry;
        self.registers.f.carry = carry;
    }
}
