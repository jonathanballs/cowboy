use crate::{
    instructions::{r16::R16, r8::R8},
    mmu::MMU,
};

use super::CPU;

impl CPU {
    // bitwise operations
    pub(super) fn and(&mut self, b: u8) {
        let result = b & self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    pub(super) fn or(&mut self, b: u8) {
        let result = b | self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
    }

    pub(super) fn xor(&mut self, b: u8) {
        let result = b ^ self.registers.a;
        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
    }

    // increment/decrement
    pub(super) fn inc_r16(&mut self, a: R16) {
        let value = self.registers.get_r16(a);
        let result = value.wrapping_add(1);
        self.registers.set_r16(a, result);
    }

    pub(super) fn dec_r16(&mut self, a: R16) {
        let value = self.registers.get_r16(a);
        let result = value.wrapping_sub(1);
        self.registers.set_r16(a, result);
    }

    pub(super) fn inc(&mut self, memory: &mut MMU, a: R8) {
        let value = self.get_r8_byte(memory, a);
        let result = value.wrapping_add(1);
        self.set_r8_byte(memory, a, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        // Half carry will occur when the lower nibble was 0b1111
        self.registers.f.half_carry = (value & 0xF) == 0xF;
    }

    pub(super) fn dec(&mut self, mmu: &mut MMU, a: R8) {
        let value = self.get_r8_byte(mmu, a);
        let result = value.wrapping_sub(1);
        self.set_r8_byte(mmu, a, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        // Half carry will occur when the lower nibble was 0b0000
        self.registers.f.half_carry = (value & 0xF) == 0x0;
    }

    // maths
    pub(super) fn add(&mut self, b: u8) {
        let result = self.registers.a.wrapping_add(b);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        // Carry if a and b go over 0xFF
        self.registers.f.carry = (self.registers.a as u16) + (b as u16) > 0xFF;
        // Half carry if the lower nibbles of a and b go over 0xF
        self.registers.f.half_carry = (self.registers.a & 0xF) + (b & 0xF) > 0xF;

        self.registers.a = result;
    }

    pub(super) fn add_r16(&mut self, a_reg: R16, b: u16) {
        let a = self.registers.get_r16(a_reg);
        let result = a.wrapping_add(b);

        self.registers.f.half_carry = (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF;
        self.registers.f.subtract = false;
        self.registers.f.carry = result < a;
        self.registers.set_r16(a_reg, result);
    }

    pub(super) fn sub(&mut self, b: u8) {
        // Use flag setting from CP
        self.cp(b);
        self.registers.a = self.registers.a.wrapping_sub(b);
    }

    pub(super) fn cp(&mut self, b: u8) {
        let result = self.registers.a.wrapping_sub(b);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (b & 0xF);
        self.registers.f.carry = self.registers.a < b;
    }
}
