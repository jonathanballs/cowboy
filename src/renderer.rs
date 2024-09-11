use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::mmu::ppu::{Tile, PPU};

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

fn render_tile(
    buffer: &mut Vec<u32>,
    tile: Tile,
    x: usize,
    y: usize,
    flags: u8,
    transparency: bool,
) {
    let x_flip = (flags & 0x20) > 0;
    let y_flip = (flags & 0x40) > 0;

    for tile_y in 0..8 {
        for tile_x in 0..8 {
            let x_offset = if x_flip { (x + 8) - tile_x } else { x + tile_x };

            let y_offset = if y_flip {
                y.wrapping_add(8).wrapping_sub(tile_y).wrapping_mul(WIDTH)
            } else {
                y.wrapping_add(tile_y).wrapping_mul(WIDTH)
            };

            if x_offset >= WIDTH || y.wrapping_add(tile_y) >= HEIGHT {
                continue;
            }

            if transparency && tile[tile_x][tile_y] == 0 {
                continue;
            }

            buffer[y_offset + x_offset] = palette(tile[tile_x][tile_y]);
        }
    }
}

fn latest_ppu(rx: &Receiver<PPU>) -> Option<PPU> {
    let mut latest_frame = None;

    loop {
        match rx.try_recv() {
            Ok(frame) => {
                // Update the latest frame
                latest_frame = Some(frame);
            }
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                panic!("");
            }
        }
    }

    latest_frame
}

pub fn window_loop(rx: Receiver<PPU>, tx: Sender<(bool, Key)>, game_title: &String) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        format!("Cowboy Emulator - {}", game_title).as_str(),
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
        if let Some(ppu) = latest_ppu(&rx) {
            for i in 0..1024 {
                let tile_index = ppu.get_byte(0x9800 + i);

                render_tile(
                    &mut buffer,
                    ppu.get_tile(tile_index),
                    (ppu.scx as usize).wrapping_add(i as usize * 8) % 256,
                    ((i as usize / 32) * 8).wrapping_sub(ppu.scy as usize),
                    0x0,
                    false,
                )
            }

            for i in 0..40 {
                let start_position = 0xFE00 + i * 4;

                let y_position = ppu.get_byte(start_position);

                let x_position = ppu.get_byte(start_position + 1);
                let tile_index = ppu.get_byte(start_position + 2);
                let flags = ppu.get_byte(start_position + 3);

                render_tile(
                    &mut buffer,
                    ppu.get_object(tile_index),
                    x_position.wrapping_sub(8) as usize,
                    y_position.wrapping_sub(16) as usize,
                    flags,
                    true,
                );
            }
        }

        // dispatch unreleased keys
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| tx.send((true, *key)).unwrap());

        // dispatch released keys
        window
            .get_keys_released()
            .iter()
            .for_each(|key| tx.send((false, *key)).unwrap());
    }
}
