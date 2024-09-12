use crate::{
    cpu::CPU,
    instructions::{r16::R16, r8::R8},
    mmu::MMU,
};

impl CPU {
    /*
     *
     * Bitwise Operations
     *
     */
    pub(in crate::cpu) fn and(&mut self, b: u8) {
        let result = b & self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    pub(in crate::cpu) fn or(&mut self, b: u8) {
        let result = b | self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
    }

    pub(in crate::cpu) fn xor(&mut self, b: u8) {
        let result = b ^ self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
    }

    /*
     *
     * Increment & Decrement
     *
     */
    pub(in crate::cpu) fn inc_r16(&mut self, a: R16) {
        let value = self.registers.get_r16(a);
        let result = value.wrapping_add(1);
        self.registers.set_r16(a, result);
    }

    pub(in crate::cpu) fn dec_r16(&mut self, a: R16) {
        let value = self.registers.get_r16(a);
        let result = value.wrapping_sub(1);
        self.registers.set_r16(a, result);
    }

    pub(in crate::cpu) fn inc(&mut self, mmu: &mut MMU, a: R8) {
        let value = self.get_r8_byte(mmu, a);
        let result = value.wrapping_add(1);
        self.set_r8_byte(mmu, a, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        // Half carry will occur when the lower nibble was 0b1111
        self.registers.f.half_carry = (value & 0xF) == 0xF;
    }

    pub(in crate::cpu) fn dec(&mut self, mmu: &mut MMU, r: R8) {
        let value = self.get_r8_byte(mmu, r);
        let result = value.wrapping_sub(1);
        self.set_r8_byte(mmu, r, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        // Half carry will occur when the lower nibble was 0b0000
        self.registers.f.half_carry = (value & 0xF) == 0x0;
    }

    /*
     *
     * Mathematic Operations
     *
     */
    pub(in crate::cpu) fn add(&mut self, b: u8) {
        let (result, carry) = self.registers.a.overflowing_add(b);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        // Carry if a and b go over 0xFF
        self.registers.f.carry = carry;
        // Half carry if the lower nibbles of a and b go over 0xF
        self.registers.f.half_carry = (self.registers.a & 0xF) + (b & 0xF) > 0xF;

        self.registers.a = result;
    }

    pub(in crate::cpu) fn add_r16(&mut self, a_reg: R16, b: u16) {
        let a = self.registers.get_r16(a_reg);
        let (result, carry) = a.overflowing_add(b);

        self.registers.f.half_carry = (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry;
        self.registers.set_r16(a_reg, result);
    }

    pub(in crate::cpu) fn adc(&mut self, value: u8) {
        let a = self.registers.a;
        let c = self.registers.f.carry as u8;
        let result = a.wrapping_add(value).wrapping_add(c);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (a as u16 & 0xF) + (value as u16 & 0xF) + c as u16 > 0xF;
        self.registers.f.carry = (a as u16) + (value as u16) + (c as u16) > 0xFF;

        self.registers.a = result;
    }

    pub(in crate::cpu) fn sub(&mut self, b: u8) {
        // Use flag setting from CP
        self.cp(b);
        self.registers.a = self.registers.a.wrapping_sub(b);
    }

    pub(in crate::cpu) fn cp(&mut self, b: u8) {
        let result = self.registers.a.wrapping_sub(b);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (b & 0xF);
        self.registers.f.carry = self.registers.a < b;
    }

    pub(in crate::cpu) fn sbc(&mut self, value: u8) {
        let a = self.registers.a;
        let c = self.registers.f.carry as u8;
        let result = a.wrapping_sub(value).wrapping_sub(c);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (a & 0xF).wrapping_sub(value & 0xF).wrapping_sub(c) > 0xF;
        self.registers.f.carry = (a as u16) < (value as u16) + (c as u16);

        self.registers.a = result;
    }
}
