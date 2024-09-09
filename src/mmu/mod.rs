use bootrom::BOOT_ROM;
use joypad::Joypad;

pub mod bootrom;
pub mod joypad;
pub mod rom;

pub struct MMU {
    boot_rom_enabled: bool,
    pub rom: Vec<u8>,
    pub ram: [u8; 0xFFFF],
    pub joypad: Joypad,
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        MMU {
            boot_rom_enabled: true,
            joypad: Joypad::new(),
            ram: [0x0; 0xFFFF],
            rom,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // Boot Rom
            0x0..=0xFF => {
                if self.boot_rom_enabled {
                    BOOT_ROM[addr as usize]
                } else {
                    *self.rom.get(addr as usize).unwrap_or(&0)
                }
            }
            0x100..=0x7FFF => *self.rom.get(addr as usize).unwrap_or(&0),
            0xC000..=0xDFFF => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),
            0xFF00 => self.joypad.read_byte(addr),
            0xFF50 => self.boot_rom_enabled as u8,

            // HRam
            0xFF80..=0xFFFE => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0xDFFF => self.ram[addr as usize - 0x8000] = value,
            0xFF00 => self.joypad.write_byte(addr, value),
            0xFF50 => self.boot_rom_enabled = value == 0,

            0xFF80..=0xFFFE => self.ram[(addr - 0x8000) as usize] = value,
            _ => unreachable!(),
        }
    }
}
