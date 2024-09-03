mod bootrom;
mod gameboy;
pub mod instructions;
mod ppu;
mod registers;
mod renderer;
mod rom;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;

use colored::*;
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

    let paused = Arc::new(AtomicBool::new(false));

    let p = paused.clone();

    ctrlc::set_handler(move || {
        if p.load(Ordering::SeqCst) {
            // If already paused, stop the emulator
            println!("{}", "\nSo long space cowboy".red());
            exit(-1);
        } else {
            // If running, pause the emulator
            p.store(true, Ordering::SeqCst);
            println!("Received Ctrl+C! Pausing at the end of this step...");
        }
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        if paused.load(Ordering::SeqCst) {
            gameboy.debugger_enabled = true;
        }

        gameboy.step();

        //paused.store(gameboy.debugger_enabled, Ordering::SeqCst);
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
