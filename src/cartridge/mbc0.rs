use crate::cartridge::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(rom: Vec<u8>) -> MBC0 {
        MBC0 { rom }
    }
}

impl MBC for MBC0 {
    fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write_byte(&mut self, _addr: u16, _value: u8) {
        ();
    }
}
