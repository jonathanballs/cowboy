use std::fmt;

use crate::instructions::cond::Cond;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    pub fn evaluate_condition(&self, cond: Cond) -> bool {
        match cond {
            Cond::Z => self.zero,
            Cond::NZ => !self.zero,
            Cond::C => self.carry,
            Cond::NC => !self.carry,
        }
    }

    pub fn as_byte(&self) -> u8 {
        ((self.zero as u8) << ZERO_FLAG_BYTE_POSITION)
            | ((self.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION)
            | ((self.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION)
            | ((self.carry as u8) << CARRY_FLAG_BYTE_POSITION)
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl fmt::Debug for FlagsRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlagsRegister")
            .field("zero", &self.zero)
            .field("subtract", &self.subtract)
            .field("half_carry", &self.half_carry)
            .field("carry", &self.carry)
            .finish()
    }
}
