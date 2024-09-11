use crate::{cpu::CPU, instructions::r8::R8, mmu::MMU};

impl CPU {
    /*
     *
     * Bit checking and setting
     *
     */
    pub(in crate::cpu) fn bit(&mut self, mmu: &MMU, r: R8, bit_index: u8) {
        let result = self.get_r8_byte(mmu, r) & (1 << bit_index);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    pub(in crate::cpu) fn res(&mut self, mmu: &mut MMU, r: R8, bit_index: u8) {
        let result = self.get_r8_byte(mmu, r) & !(1 << bit_index);
        self.set_r8_byte(mmu, r, result);
    }

    pub(in crate::cpu) fn set(&mut self, mmu: &mut MMU, r: R8, bit_index: u8) {
        let result = self.get_r8_byte(mmu, r) | 1 << bit_index;
        self.set_r8_byte(mmu, r, result);
    }

    /*
     *
     * Bit rotation
     *
     */
    pub(in crate::cpu) fn swap(&mut self, mmu: &mut MMU, r: R8) {
        let register_value = self.get_r8_byte(mmu, r);
        let result = (register_value >> 4) | (register_value << 4);
        self.set_r8_byte(mmu, r, result);
        self.registers.f.zero = result == 0;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
    }

    pub(in crate::cpu) fn rl(&mut self, mmu: &mut MMU, r: R8) {
        let value = self.get_r8_byte(mmu, r);
        let result = (value << 1) | self.registers.f.carry as u8;
        self.set_r8_byte(mmu, r, result);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value >> 7 == 1;
    }

    pub(in crate::cpu) fn rla(&mut self, mmu: &mut MMU) {
        self.rl(mmu, R8::A);
        self.registers.f.zero = false;
    }

    pub(in crate::cpu) fn srl(&mut self, mmu: &mut MMU, reg: R8) {
        let value = self.get_r8_byte(mmu, reg);
        let result = value >> 1;
        self.set_r8_byte(mmu, reg, result);
        self.registers.f.carry = value & 1 == 1;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
        self.registers.f.zero = result == 0;
    }

    pub(in crate::cpu) fn sla(&mut self, mmu: &mut MMU, reg: R8) {
        let value = self.get_r8_byte(mmu, R8::A);
        let new_value = (value << 1) | self.registers.f.carry as u8;
        self.set_r8_byte(mmu, reg, new_value);
        self.registers.f.carry = value >> 7 == 1;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
        self.registers.f.zero = new_value == 0;
    }

    pub(in crate::cpu) fn rlca(&mut self) {
        let value = self.registers.a;
        self.registers.f.carry = value >> 7 > 0;
        self.registers.f.zero = false;
        self.registers.f.half_carry = false;
        self.registers.f.subtract = false;
        self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
    }

    pub(in crate::cpu) fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.half_carry = true;
        self.registers.f.subtract = true;
    }

    pub(in crate::cpu) fn ccf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    pub(in crate::cpu) fn scf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    pub(in crate::cpu) fn daa(&mut self) {
        let mut correction = 0;
        let mut set_carry = false;

        if self.registers.f.half_carry
            || (!self.registers.f.subtract && (self.registers.a & 0xf) > 9)
        {
            correction |= 0x6;
        }

        if self.registers.f.carry || (!self.registers.f.subtract && self.registers.a > 0x99) {
            correction |= 0x60;
            set_carry = true;
        }

        if self.registers.f.subtract {
            self.registers.a = self.registers.a.wrapping_sub(correction);
        } else {
            self.registers.a = self.registers.a.wrapping_add(correction);
        }

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.half_carry = false;
        self.registers.f.carry = set_carry;
    }
}
