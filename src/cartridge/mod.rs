use header::{CartridgeHeader, CartridgeType};
use mbc0::MBC0;

pub mod header;
mod mbc0;

pub trait MBC {
    fn read_byte(&self, addr: u16) -> u8;
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
            _ => unreachable!(),
        };

        Cartridge { header, mbc }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mbc.read_byte(addr)
    }
}
