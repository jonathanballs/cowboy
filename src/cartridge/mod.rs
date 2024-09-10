use header::{CartridgeHeader, CartridgeType};
use mbc0::MBC0;
use mbc1::MBC1;

pub mod header;

mod mbc0;
mod mbc1;

pub trait MBC {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
}

pub struct Cartridge {
    pub header: CartridgeHeader,
    mbc: Box<dyn MBC>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Cartridge {
        let header = CartridgeHeader::new(&rom).unwrap();

        let mbc: Box<dyn MBC> = match header.cartridge_type() {
            CartridgeType::RomOnly => Box::new(MBC0::new(rom)),
            CartridgeType::Mbc1 => Box::new(MBC1::new(rom)),
            _ => {
                dbg!(header.cartridge_type());
                todo!()
            }
        };

        Cartridge { header, mbc }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mbc.read_byte(addr)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.mbc.write_byte(addr, value)
    }
}
