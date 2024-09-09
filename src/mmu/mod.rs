use joypad::Joypad;

pub mod bootrom;
pub mod joypad;
pub mod rom;

pub struct MMU {
    pub joypad: Joypad,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            joypad: Joypad::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad.read_byte(addr),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF00 => self.joypad.write_byte(addr, value),
            _ => unreachable!(),
        }
    }
}
