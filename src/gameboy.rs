use std::fmt;

use crate::{registers::Registers, rom::GBCHeader};

pub struct GameBoy {
    pub registers: Registers,
    pub rom_data: Vec<u8>,
}

impl GameBoy {
    pub fn new(rom_data: Vec<u8>) -> GameBoy {
        GameBoy {
            registers: Registers::new(),
            rom_data,
        }
    }

    pub fn get_rom_header(&self) -> Result<GBCHeader, &'static str> {
        GBCHeader::new(&self.rom_data)
    }
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("registers", &self.registers)
            .field("rom", &self.get_rom_header())
            .finish()
    }
}
