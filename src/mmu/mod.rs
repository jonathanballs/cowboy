pub mod bootrom;
pub mod joypad;
pub mod ppu;
pub mod timer;

use crate::cartridge::Cartridge;
use bootrom::BOOT_ROM;
use joypad::Joypad;
use ppu::PPU;
use timer::Timer;

pub struct MMU {
    boot_rom_enabled: bool,
    pub cartridge: Cartridge,
    pub ram: [u8; 0xFFFF],
    pub joypad: Joypad,
    pub ppu: PPU,
    pub timer: Timer,
    pub ie: u8,
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        MMU {
            cartridge: Cartridge::new(rom),
            boot_rom_enabled: true,
            joypad: Joypad::new(),
            ram: [0x0; 0xFFFF],
            ppu: PPU::new(),
            ie: 0,

            timer: Timer::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // Boot rom or regular rom
            0x0..=0xFF => {
                if self.boot_rom_enabled {
                    BOOT_ROM[addr as usize]
                } else {
                    self.cartridge.read_byte(addr)
                }
            }

            // ROM
            0x100..=0x7FFF => self.cartridge.read_byte(addr),

            // VRAM
            0x8000..=0x9FFF => self.ppu.get_byte(addr),

            // Cartridge RAM
            0xA000..=0xBFFF => self.cartridge.read_byte(addr),

            // Working RAM
            0xC000..=0xDFFF => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),

            // Echo RAM
            0xE000..=0xFDFF => {
                println!("tried to read echo ram");
                dbg!(addr);
                unreachable!()
            }

            // Joypad
            0xFF00 => self.joypad.read_byte(addr),

            // Interrupt registers
            0xFF04..=0xFF07 => self.timer.read_byte(addr),

            // Boot rom enabled
            0xFF50 => self.boot_rom_enabled as u8,

            // VBlank interrupt
            0xFF0F => {
                (self.ppu.vblank_irq as u8)
                    | ((self.timer.timer_irq as u8) << 2)
                    | ((self.joypad.joypad_irq as u8) << 4)
            }

            // VOAM
            0xFE00..=0xFF7F => self.ppu.get_byte(addr),

            // HRam
            0xFF80..=0xFFFE => *self.ram.get((addr - 0x8000) as usize).unwrap_or(&0),

            // Interrupt enable
            0xFFFF => self.ie,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM bank - ignore
            0x0..=0x7FFF => self.cartridge.write_byte(addr, value),

            // VRAM
            0x8000..=0x9FFF => self.ppu.set_byte(addr, value),

            // Cartridge RAM
            0xA000..=0xBFFF => self.cartridge.write_byte(addr, value),

            // Working RAM
            0xC000..=0xDFFF => self.ram[addr as usize - 0x8000] = value,

            // Joypad
            0xFF00 => self.joypad.write_byte(addr, value),

            // Serial transfer - currently unsupported
            0xFF01..=0xFF02 => (),

            // Timer
            0xFF04..=0xFF07 => self.timer.write_byte(addr, value),

            // Interrupt
            0xFF0F => self.ppu.vblank_irq = value & 0x1 > 0,

            // DMA transfer
            0xFF46 => {
                let source_addr = (value as u16) << 8;
                for offset in 0x0..=0x9F {
                    self.write_byte(0xFE00 + offset, self.read_byte(source_addr + offset))
                }
            }

            // Enable boot rom
            0xFF50 => self.boot_rom_enabled = value == 0,

            // Not usable. Ignore writes...
            0xFEA0..=0xFEFF => (),

            0xFE00..=0xFF7F => self.ppu.set_byte(addr, value),

            // HRAM
            0xFF80..=0xFFFE => self.ram[(addr - 0x8000) as usize] = value,

            // Interrupt enable
            0xFFFF => self.ie = value,

            _ => {
                dbg!(addr);
                unreachable!()
            }
        }
    }
}
