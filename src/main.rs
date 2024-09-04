mod bootrom;
mod gameboy;
pub mod instructions;
mod ppu;
mod registers;
mod renderer;
mod rom;

use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

use gameboy::GameBoy;
use minifb::Key;
use ppu::PPU;
use renderer::window_loop;

fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let (tx, rx) = mpsc::channel::<PPU>();
    let (tx_key, rx_key) = mpsc::channel::<(bool, Key)>();

    let emulator_loop = thread::spawn(move || {
        let rom_data = read_file_to_bytes("roms/tetris.gb").unwrap();
        let mut gameboy = GameBoy::new(rom_data, tx.clone(), rx_key);
        gameboy.start()
    });

    window_loop(rx, tx_key);

    let _ = emulator_loop.join();
}
