use std::fmt;

use crate::{
    debugger::is_gameboy_doctor,
    instructions::{r16::R16, r16mem::R16mem, r16stk::R16stk, r8::R8},
};

use super::flag_register::FlagsRegister;

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
        if is_gameboy_doctor() {
            Registers {
                a: 0x1,
                f: FlagsRegister::from(0xB0),
                b: 0,
                c: 0x13,
                d: 0,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                sp: 0xFFFE,
                pc: 0x100,
            }
        } else {
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

    pub fn get_r16_stk(&self, register: R16stk) -> u16 {
        match register {
            R16stk::BC => self.get_r16(R16::BC),
            R16stk::DE => self.get_r16(R16::DE),
            R16stk::HL => self.get_r16(R16::HL),
            R16stk::AF => ((self.a as u16) << 8) | (self.f.as_byte() as u16),
        }
    }

    pub fn set_r16_stk(&mut self, register: R16stk, value: u16) {
        match register {
            R16stk::BC => self.set_r16(R16::BC, value),
            R16stk::DE => self.set_r16(R16::DE, value),
            R16stk::HL => self.set_r16(R16::HL, value),
            R16stk::AF => {
                self.a = (value >> 8) as u8;
                self.f = FlagsRegister::from((value & 0xFF) as u8);
            }
        }
    }

    pub fn get_r8(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
            _ => unreachable!(),
        }
    }

    pub fn set_r8(&mut self, register: R8, value: u8) {
        match register {
            R8::A => self.a = value,
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
            _ => unreachable!(),
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
