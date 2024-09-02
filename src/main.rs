mod bootrom;
mod gameboy;
pub mod instructions;
mod ppu;
mod registers;
mod rom;

use std::io::Read;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::{fs::File, sync::mpsc::Receiver};

use gameboy::GameBoy;
use minifb::{Key, Window, WindowOptions};
use ppu::PPUState;

const WIDTH: usize = 160 * 4;
const HEIGHT: usize = 144 * 4;

fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn emulator_loop(tx: Sender<PPUState>) {
    let rom_data = read_file_to_bytes("tetris.gb").unwrap();
    let mut gameboy = GameBoy::new(rom_data, tx);

    loop {
        gameboy.step();
    }
}

fn window_loop(rx: Receiver<PPUState>) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Cowboy Emulator", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to
        // handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Receive frame buffer from the emulator
        match rx.recv() {
            Ok(ppu_state) => {
                println!("{:?}", ppu_state);
            }
            _ => (),
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<PPUState>();

    let emulator_loop = thread::spawn(move || {
        emulator_loop(tx);
    });

    window_loop(rx);

    let _ = emulator_loop.join();
}
