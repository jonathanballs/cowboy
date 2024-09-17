use std::{
    fmt, thread,
    time::{Duration, Instant},
};

use crate::debugger::is_gameboy_doctor;

const VRAM_SIZE: usize = 0x2000;
const VOAM_SIZE: usize = 0xA0;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const TARGET_FPS: f64 = 60.0;

pub type Tile = [[u8; 8]; 8];

#[derive(Clone)]
pub struct PPU {
    pub frame_buffer: Vec<u32>,

    frame_available: bool,
    frame_number: u32,
    last_frame_time: Instant,
    pub vblank_irq: bool,
    pub stat_irq: bool,

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
            frame_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            frame_available: false,
            frame_number: 1,
            last_frame_time: Instant::now(),
            vblank_irq: false,
            stat_irq: false,

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
                if self.ly < SCREEN_HEIGHT as u8 {
                    self.render_scanline(self.ly);
                }

                self.modeclock -= 456;
                self.ly = (self.ly + 1) % 154;

                // Enter mode 1 (VBLANK)
                if self.ly == 144 {
                    self.vblank_irq = true
                }

                if self.ly == self.lyc && self.stat & 0x40 == 0x40 {
                    self.stat_irq = true;
                }

                // Frame finished - flush to screen
                if self.ly == 0 {
                    // Calculate how long to sleep
                    let elapsed = self.last_frame_time.elapsed();
                    let frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);

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
            0xFF41 => {
                let mut ret = self.stat & 0xF8;
                if self.ly == self.lyc {
                    ret |= 0x4;
                }

                // just put it into mode 3...

                return ret;
            }
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => {
                if is_gameboy_doctor() {
                    0x90
                } else {
                    self.ly
                }
            }
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,

            0xFF4A => self.wy,
            0xFF4B => self.wx,

            0xFF4D => 0,

            0xFF48 => self.obj_palette_0,
            0xFF49 => self.obj_palette_1,

            0xFE00..=0xFE9F => self.voam[(addr - 0xFE00) as usize],

            _ => {
                println!("tried to read {:#04x}", addr);
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
            0xFF4D => (),

            0xFe00..=0xFE9F => self.voam[(addr - 0xFE00) as usize] = value,

            0xFF7F => (),

            _ => {
                println!("tried to write {:#04x}", addr);
                todo!()
            }
        }
    }

    pub fn get_and_reset_frame_available(&mut self) -> bool {
        let result = self.frame_available;
        self.frame_available = false;
        return result;
    }

    pub fn get_tile_pixel(
        &self,
        tile_index: u8,
        line_index: u16,
        col_index: u16,
        use_8000: bool,
    ) -> u8 {
        let start_address = if self.lcdc & 0x10 > 0 || use_8000 {
            0x8000 + ((tile_index as u16) * 16) as u16
        } else {
            let offset = ((tile_index as i8) as i16) * 16;
            0x9000_u16.wrapping_add(offset as u16)
        };

        let byte_a = self.get_byte(start_address + (2 * line_index));
        let byte_b = self.get_byte(start_address + (2 * line_index) + 1);

        let bit1 = (byte_a >> 7 - col_index) & 1;
        let bit2 = (byte_b >> 7 - col_index) & 1;

        return ((bit2 << 1) | bit1) as u8;
    }

    pub fn get_object(&self, tile_index: u8) -> Tile {
        let start_address = 0x8000 + ((tile_index as u16) * 16) as u16;
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

    fn palette(&self, id: u8) -> u32 {
        match id {
            0x0 => 0xFFFFFFFF,
            0x1 => 0xFF666666,
            0x2 => 0xFFBBBBBB,
            0x3 => 0xFF000000,
            _ => unreachable!(),
        }
    }

    fn render_objects(&mut self, line: u8) {
        if self.lcdc & 0x2 != 0x2 {
            return;
        }

        let buffer_y_offset = (line as usize) * SCREEN_WIDTH;
        let mut num_objects_rendered = 0;

        for i in 0..40 {
            let mut did_render_object = false;
            let start_position = 0xFE00 + i * 4;

            let object_y = self.get_byte(start_position);
            let object_line = line.wrapping_sub(object_y).wrapping_add(16);
            if object_line >= 8 {
                continue;
            }
            let x_position = self.get_byte(start_position + 1);
            let tile_index = self.get_byte(start_position + 2);
            let flags = self.get_byte(start_position + 3);

            let x_flip = flags & 0x20 == 0x20;
            let y_flip = flags & 0x40 == 0x40;

            for x_offset in 0..8 {
                let x_position = if x_flip {
                    (x_position as usize).wrapping_sub(x_offset as usize + 1)
                } else {
                    (x_position as usize)
                        .wrapping_add(x_offset as usize)
                        .wrapping_sub(8)
                };

                if x_position >= SCREEN_WIDTH {
                    continue;
                }

                let tile_line = if y_flip { 8 - object_line } else { object_line };

                let tile_pixel = self.get_tile_pixel(tile_index, tile_line as u16, x_offset, true);

                if tile_pixel == 0 {
                    continue;
                }

                self.frame_buffer[buffer_y_offset + x_position] = self.palette(tile_pixel);
                did_render_object = true;
            }

            if did_render_object {
                num_objects_rendered += 1;
            }

            if num_objects_rendered >= 10 {
                break;
            }
        }
    }

    fn render_background(&mut self, line: u8) {
        if self.lcdc & 0x1 != 0x1 {
            return;
        }

        let buffer_y_offset = (line as usize) * SCREEN_WIDTH;

        // draw background
        let background_y = line.wrapping_add(self.scy);
        let tile_map_row = background_y / 8;
        for buffer_x_offset in 0..SCREEN_WIDTH {
            let background_x = (buffer_x_offset + self.scx as usize) % 256;

            let tile_map_index = (tile_map_row as usize * 32) + (background_x / 8);
            let tile_map_data_area = if self.lcdc & 0x8 == 0x8 {
                0x9C00
            } else {
                0x9800
            };
            let tile_index = self.get_byte(tile_map_data_area + tile_map_index as u16);

            let tile_line = background_y % 8;
            let tile_col = background_x % 8;

            let tile_pixel =
                self.get_tile_pixel(tile_index, tile_line as u16, tile_col as u16, false);

            self.frame_buffer[buffer_y_offset + buffer_x_offset] = self.palette(tile_pixel);
        }
    }

    fn render_window(&mut self, line: u8) {
        let buffer_y_offset = (line as usize) * SCREEN_WIDTH;
        if self.lcdc & 0x21 != 0x21 {
            return;
        }

        // draw window
        for buffer_x_offset in 0..SCREEN_WIDTH {
            let window_x = buffer_x_offset
                .wrapping_sub(self.wx as usize)
                .wrapping_add(7);
            if window_x >= SCREEN_WIDTH {
                continue;
            };

            let window_y = (line as usize).wrapping_sub(self.wy as usize);
            if window_y as usize >= SCREEN_HEIGHT {
                continue;
            };

            let tile_map_index = ((window_y / 8) as usize * 32) + (window_x / 8);
            let tile_map_data_area = if self.lcdc & 0x40 == 0x40 {
                0x9C00
            } else {
                0x9800
            };
            let tile_index = self.get_byte(tile_map_data_area + tile_map_index as u16);

            let tile_line = window_y % 8;
            let tile_col = window_x % 8;

            let tile_pixel =
                self.get_tile_pixel(tile_index, tile_line as u16, tile_col as u16, false);

            self.frame_buffer[buffer_y_offset + buffer_x_offset] = self.palette(tile_pixel);
        }
    }

    fn render_scanline(&mut self, line: u8) {
        // calculate background scanline
        self.render_background(line);
        self.render_window(line);
        self.render_objects(line);
    }
}

impl fmt::Debug for PPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PPU")
            .field("scy", &self.scy)
            .field("scx", &self.scx)
            .field("ly", &self.ly)
            .field("lyc", &self.lyc)
            .field("wy", &self.wy)
            .field("wx", &self.wx)
            .field("lcdc", &self.lcdc)
            .field("bgp", &self.bgp)
            .field("modeclock", &self.modeclock)
            .finish()
    }
}
