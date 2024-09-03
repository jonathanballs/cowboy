use minifb::{Key, Scale, Window, WindowOptions};
use std::sync::mpsc::Receiver;

use crate::ppu::{Tile, PPU};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn palette(id: u8) -> u32 {
    match id {
        0x0 => 0xFFFFFFFF,
        0x1 => 0xFF666666,
        0x2 => 0xFFBBBBBB,
        0x3 => 0xFF000000,
        _ => unreachable!(),
    }
}

fn render_tile(tile: Tile, buffer: &mut Vec<u32>, x: usize, y: usize) {
    for tile_y in 0..8 {
        for tile_x in 0..8 {
            let x_offset = x + tile_x;
            let y_offset = y.wrapping_add(tile_y).wrapping_mul(WIDTH);

            if x_offset >= WIDTH || y.wrapping_add(tile_y) >= HEIGHT {
                continue;
            }

            buffer[y_offset + x_offset] = palette(tile[tile_x][tile_y]);
        }
    }
}

pub fn window_loop(rx: Receiver<PPU>) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Cowboy Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to
        // handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Receive frame buffer from the emulator
        match rx.recv() {
            Ok(ppu) => {
                for i in 0..1024 {
                    let tile_index = ppu.get_byte(0x9800 + i);

                    render_tile(
                        ppu.get_tile(tile_index as usize),
                        &mut buffer,
                        (i as usize * 8) % 256,
                        ((i as usize / 32) * 8).wrapping_sub(ppu.scy as usize),
                    )
                }

                for i in 0..40 {
                    let start_position = 0xFE00 + i * 4;

                    let y_position = ppu.get_byte(start_position);

                    let x_position = ppu.get_byte(start_position + 1);
                    let tile_index = ppu.get_byte(start_position + 2);
                    let flags = ppu.get_byte(start_position + 3);

                    render_tile(
                        ppu.get_tile(tile_index as usize),
                        &mut buffer,
                        x_position.wrapping_sub(8) as usize,
                        y_position.wrapping_sub(16) as usize,
                    );
                }
            }
            _ => (),
        }
    }
}
