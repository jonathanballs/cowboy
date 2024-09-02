use std::sync::mpsc::Sender;

const VRAM_SIZE: usize = 0x2000;
const VOAM_SIZE: usize = 0xA0;

pub type Tile = [[u8; 8]; 8];

#[derive(Debug, Clone)]
pub struct PPU {
    tx: Sender<PPU>,
    frame_number: u32,
    vram: [u8; VRAM_SIZE],
    voam: [u8; VOAM_SIZE],
    scy: u8,
    scx: u8,
    line: u8,
    lcdc: u8,
    bgp: u8,
    modeclock: u32,
}

impl PPU {
    pub fn new(tx: Sender<PPU>) -> PPU {
        PPU {
            scy: 0,
            scx: 0,
            bgp: 0,
            line: 0,
            lcdc: 0,
            modeclock: 32,
            frame_number: 1,
            vram: [0; VRAM_SIZE],
            voam: [0; VOAM_SIZE],
            tx,
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
                self.line = (self.line + 1) % 154;
                if self.line == 0 {
                    self.frame_number += 1;
                    self.tx.send(self.clone()).unwrap();
                }
            }
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            // VRAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],

            // Sound.
            0xFF10..=0xFF26 => return 0,

            0xFF40 => self.lcdc,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF47 => self.bgp,
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
            0xFF10..=0xFF26 => (),

            0xFF40 => self.lcdc = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF47 => self.bgp = value,

            _ => {
                println!("PPU does not support address {:x?}", addr);
                todo!()
            }
        }
    }

    pub fn get_tile(&self, tile_index: usize) -> Tile {
        let start_address = 0x8000 + (tile_index * 16) as u16;
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
