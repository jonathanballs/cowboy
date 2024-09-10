use std::fmt;
use std::slice;

pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
}

#[repr(C, packed)]
pub struct CartridgeHeader {
    padding: [u8; 0x100],
    entry_point: [u8; 4],
    nintendo_logo: [u8; 48],
    title: [u8; 11],
    manufacturer_code: [u8; 4],
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom: u8,
    header_checksum: u8,
    global_checksum: [u8; 2],
}

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 0x150 {
            return Err("Data is too short for a valid GBC header");
        }

        Ok(unsafe { std::ptr::read(data.as_ptr() as *const CartridgeHeader) })
    }

    pub fn title(&self) -> String {
        String::from_utf8_lossy(&self.title)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn cartridge_type(&self) -> CartridgeType {
        match self.cartridge_type {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::Mbc1,
            0x02 => CartridgeType::Mbc1Ram,
            0x03 => CartridgeType::Mbc1RamBattery,

            _ => todo!(),
        }
    }

    pub fn cartridge_type_name(&self) -> &'static str {
        match self.cartridge_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            // Add more cartridge types as needed
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            _ => "Unknown",
        }
    }

    pub fn rom_size_str(&self) -> String {
        format!("{} Kib", 32 * (1 << self.rom_size))
    }

    pub fn ram_size_str(&self) -> &'static str {
        match self.ram_size {
            0x00 => "No RAM",
            0x02 => "8 KiB",
            0x03 => "32 KiB",
            0x04 => "128 KiB",
            0x05 => "64 KiB",
            _ => "Unknown",
        }
    }

    pub fn validate_header_checksum(&self) -> bool {
        let bytes = unsafe {
            slice::from_raw_parts((self as *const Self as *const u8).add(0x134), 0x14D - 0x134)
        };

        let checksum = bytes
            .iter()
            .fold(0u8, |acc, &byte| acc.wrapping_sub(byte).wrapping_sub(1));

        checksum == self.header_checksum
    }
}

impl fmt::Debug for CartridgeHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GBCHeader")
            .field("title", &self.title())
            .field("cartridge_type", &self.cartridge_type_name())
            .field("rom_size", &self.rom_size_str())
            .field("ram_size", &self.ram_size_str())
            .field("checksum_valid?", &self.validate_header_checksum())
            .finish()
    }
}
