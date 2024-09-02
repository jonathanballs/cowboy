mod bootrom;
mod gameboy;
pub mod instructions;
mod ppu;
mod registers;
mod renderer;
mod rom;

use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{self, Sender};
use std::thread;

use gameboy::GameBoy;
use ppu::PPU;
use renderer::window_loop;

fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn emulator_loop(tx: Sender<PPU>) {
    let rom_data = read_file_to_bytes("tetris.gb").unwrap();
    let mut gameboy = GameBoy::new(rom_data, tx);

    loop {
        gameboy.step();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<PPU>();

    let emulator_loop = thread::spawn(move || {
        emulator_loop(tx);
    });

    window_loop(rx);

    let _ = emulator_loop.join();
}
