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
            buffer[((y + tile_y) * WIDTH) + x + tile_x] = palette(tile[tile_x][tile_y]);
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

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to
        // handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Receive frame buffer from the emulator
        match rx.recv() {
            Ok(ppu) => {
                for i in 0..128 {
                    render_tile(ppu.get_tile(i), &mut buffer, ((i * 8) % 128), (i / 16) * 8)
                }
            }
            _ => (),
        }
    }
}
