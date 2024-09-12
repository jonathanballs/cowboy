use crate::{
    cpu::CPU,
    instructions::{cond::Cond, r16stk::R16stk},
    mmu::MMU,
};

impl CPU {
    /*
     *
     * Jumps
     *
     */
    pub(in crate::cpu) fn jp(&mut self, addr: u16) {
        self.registers.pc = addr;
    }

    pub(in crate::cpu) fn jp_cond(&mut self, addr: u16, cond: Cond) {
        if self.registers.f.evaluate_condition(cond) {
            self.jp(addr)
        }
    }

    pub(in crate::cpu) fn jr(&mut self, offset: i8) {
        self.registers.pc = self.registers.pc.wrapping_add((offset as i16) as u16);
    }

    pub(in crate::cpu) fn jr_cond(&mut self, offset: i8, cond: Cond) {
        if self.registers.f.evaluate_condition(cond) {
            self.jr(offset)
        }
    }

    /*
     *
     * Call and Return
     *
     */
    //pub(in crate::cpu) fn call(&mut self, mmu: &mut MMU, addr: u16) {
    //    self.set_memory_word(mmu, self.registers.sp - 2, self.registers.pc + 3);
    //    self.registers.sp -= 2;
    //    self.registers.pc = addr;
    //}
    //
    //pub(in crate::cpu) fn call_cond(&mut self, mmu: &mut MMU, cond: Cond, addr: u16) {
    //    if self.registers.f.evaluate_condition(cond) {
    //        self.call(mmu, addr)
    //    }
    //}

    pub(in crate::cpu) fn rst_tgt3(&mut self, mmu: &mut MMU, addr: u16) {
        self.set_memory_word(mmu, self.registers.sp - 2, self.registers.pc + 1);
        self.registers.sp -= 2;
        self.registers.pc = addr - 1
    }

    pub(in crate::cpu) fn ret(&mut self, mmu: &mut MMU) {
        self.registers.pc = self.get_memory_word(mmu, self.registers.sp) - 1;
        self.registers.sp += 2;
    }

    pub(in crate::cpu) fn ret_cond(&mut self, mmu: &mut MMU, cond: Cond) {
        if self.registers.f.evaluate_condition(cond) {
            self.registers.pc = self.get_memory_word(mmu, self.registers.sp) - 1;
            self.registers.sp += 2;
        }
    }

    pub(in crate::cpu) fn reti(&mut self, mmu: &mut MMU) {
        self.ret(mmu);
        self.ime = true;
    }

    /*
     *
     * Push and pop
     *
     */
    pub(in crate::cpu) fn push(&mut self, mmu: &mut MMU, value: u16) {
        self.set_memory_word(mmu, self.registers.sp - 2, value);
        self.registers.sp = self.registers.sp.wrapping_sub(2);
    }

    pub(in crate::cpu) fn pop(&mut self, mmu: &mut MMU, reg: R16stk) {
        let value = self.get_memory_word(mmu, self.registers.sp);
        self.registers.set_r16_stk(reg, value);
        self.registers.sp = self.registers.sp.wrapping_add(2);
    }
}
