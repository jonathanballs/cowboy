use std::{
    thread,
    time::{Duration, Instant},
};

const VRAM_SIZE: usize = 0x2000;
const VOAM_SIZE: usize = 0xA0;

pub type Tile = [[u8; 8]; 8];

#[derive(Debug, Clone)]
pub struct PPU {
    frame_available: bool,
    frame_number: u32,
    last_frame_time: Instant,
    pub vblank_irq: bool,

    vram: [u8; VRAM_SIZE],
    voam: [u8; VOAM_SIZE],

    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub lcdc: u8,
    pub bgp: u8,
    pub modeclock: u32,

    pub wy: u8,
    pub wx: u8,

    pub obj_palette_0: u8,
    pub obj_palette_1: u8,

    pub stat: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            frame_available: false,
            frame_number: 1,
            last_frame_time: Instant::now(),
            vblank_irq: false,

            scy: 0,
            scx: 0,
            bgp: 0,
            ly: 0,
            lyc: 0,
            lcdc: 0,
            modeclock: 32,
            stat: 0,

            wy: 1,
            wx: 1,

            obj_palette_0: 0,
            obj_palette_1: 0,

            vram: [0; VRAM_SIZE],
            voam: [0; VOAM_SIZE],
        }
    }

    pub fn do_cycle(&mut self, ticks: u32) {
        let mut ticksleft = ticks;

        while ticksleft > 0 {
            let curticks = if ticksleft >= 80 { 80 } else { ticksleft };
            self.modeclock += curticks;
            ticksleft -= curticks;

            // Full line takes 114 ticks
            if self.modeclock >= 456 {
                self.modeclock -= 456;
                self.ly = (self.ly + 1) % 154;

                // Enter mode 1 (VBLANK)
                if self.ly == 144 {
                    self.vblank_irq = true
                }

                // Frame finished - flush to screen
                if self.ly == 0 {
                    // Calculate how long to sleep
                    let elapsed = self.last_frame_time.elapsed();
                    let frame_duration = Duration::from_secs_f64(1.0 / 240.0);

                    if elapsed < frame_duration {
                        thread::sleep(frame_duration - elapsed);
                    }

                    // Update last frame time
                    self.last_frame_time = Instant::now();

                    self.frame_number += 1;
                    self.frame_available = true;
                }
            }
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            // VRAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],

            // Sound.
            0xFF10..=0xFF3F => return 0,

            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,

            0xFF4A => self.wy,
            0xFF4B => self.wx,

            0xFF48 => self.obj_palette_0,
            0xFF49 => self.obj_palette_1,

            0xFe00..=0xFE9F => self.voam[(addr - 0xFE00) as usize],

            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // VRAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,

            // Sound.
            0xFF10..=0xFF3F => (),

            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obj_palette_0 = value,
            0xFF49 => self.obj_palette_1 = value,

            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

            0xFe00..=0xFE9F => self.voam[(addr - 0xFE00) as usize] = value,

            0xFF7F => (),

            _ => {
                println!("PPU does not support address {:x?}", addr);
                todo!()
            }
        }
    }

    pub fn get_and_reset_frame_available(&mut self) -> bool {
        let result = self.frame_available;
        self.frame_available = false;
        return result;
    }

    pub fn get_tile(&self, tile_index: u8) -> Tile {
        let start_address = if self.lcdc & 0x10 > 0 {
            0x8000 + ((tile_index as u16) * 16) as u16
        } else {
            let offset = ((tile_index as i8) as i16) * 16;
            0x9000_u16.wrapping_add(offset as u16)
        };

        let mut ret = [[0u8; 8]; 8];

        for i in 0..8 {
            let byte_a = self.get_byte(start_address + (2 * i));
            let byte_b = self.get_byte(start_address + (2 * i) + 1);

            for j in 0..8 {
                let bit1 = (byte_a >> 7 - j) & 1;
                let bit2 = (byte_b >> 7 - j) & 1;

                ret[j as usize][i as usize] = ((bit2 << 1) | bit1) as u8;
            }
        }

        ret
    }
}
