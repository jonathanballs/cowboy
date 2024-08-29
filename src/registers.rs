use std::fmt;

use crate::instructions::{r16::R16, r16mem::R16mem, r8::R8};

pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

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
            .field("subtract", &self.zero)
            .field("half_carry", &self.zero)
            .field("carry", &self.zero)
            .finish()
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::from(0),
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn set_r16(&mut self, register: R16, value: u16) {
        match register {
            R16::SP => self.sp = value,
            R16::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8
            }

            R16::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8
            }
            R16::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8
            }
        }
    }

    pub fn get_r16(&self, register: R16) -> u16 {
        match register {
            R16::SP => self.sp,
            R16::BC => ((self.b as u16) << 8) | (self.c as u16),
            R16::DE => ((self.d as u16) << 8) | (self.e as u16),
            R16::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    pub fn get_r16_mem(&mut self, register: R16mem) -> u16 {
        match register {
            R16mem::BC => self.get_r16(R16::BC),
            R16mem::DE => self.get_r16(R16::DE),
            R16mem::HLI => {
                let hl = self.get_r16(R16::HL);
                self.set_r16(R16::HL, hl + 1);
                hl
            }
            R16mem::HLD => {
                let hl = self.get_r16(R16::HL);
                self.set_r16(R16::HL, hl - 1);
                hl
            }
        }
    }

    pub fn get_r8(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            _ => todo!(),
        }
    }

    pub fn set_r8(&mut self, register: R8, value: u8) {
        match register {
            R8::A => self.a = value,
            _ => todo!(),
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("a", &format_args!("0x{:X}", self.a))
            .field("b", &format_args!("0x{:X}", self.b))
            .field("c", &format_args!("0x{:X}", self.c))
            .field("d", &format_args!("0x{:X}", self.d))
            .field("e", &format_args!("0x{:X}", self.e))
            .field("f", &self.f)
            .field("h", &format_args!("0x{:X}", self.h))
            .field("l", &format_args!("0x{:X}", self.l))
            .field("sp", &format_args!("0x{:X}", self.sp))
            .field("pc", &format_args!("0x{:X}", self.pc))
            .finish()
    }
}
