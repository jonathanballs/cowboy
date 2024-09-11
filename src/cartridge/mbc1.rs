use crate::cartridge::MBC;

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank: u8,
    ram: [u8; 0x2000],
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> MBC1 {
        MBC1 {
            rom,
            rom_bank: 0,
            ram: [0; 0x2000],
        }
    }
}

impl MBC for MBC1 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let bank: u16 = if self.rom_bank == 0 {
                    1
                } else {
                    self.rom_bank as u16
                };

                let idx = bank * 0x4000 | (addr & 0x3FFF);

                return self.rom[idx as usize];
            }
            0xA000..=0xBFFF => self.ram[addr as usize - 0xA000],
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0..=0x1FFF => (), // enable ram in theory
            0x2000..=0x3FFF => self.rom_bank = value & 0x1F,
            0xA000..=0xBFFF => self.ram[addr as usize - 0xA000] = value,
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }
}
