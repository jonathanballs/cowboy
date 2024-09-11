pub mod cartridge;
pub mod cpu;
pub mod debugger;
pub mod gameboy;
pub mod instructions;
pub mod mmu;
mod renderer;

use cartridge::header::CartridgeHeader;
use colored::*;
use debugger::{enable_debug, is_debug_enabled};
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use gameboy::GameBoy;
use minifb::Key;
use mmu::ppu::PPU;
use renderer::window_loop;

pub static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

fn main() {
    let (tx, rx) = mpsc::channel::<PPU>();
    let (tx_key, rx_key) = mpsc::channel::<(bool, Key)>();
    let rom = read_file_to_bytes("roms/super-mario-land.gb").unwrap();
    let game_title = CartridgeHeader::new(&rom).unwrap().title();

    let _ = thread::spawn(move || emulator_loop(rom, tx, rx_key));
    window_loop(rx, tx_key, &game_title);
}

fn emulator_loop(rom: Vec<u8>, tx: Sender<PPU>, rx: Receiver<(bool, Key)>) {
    let mut gameboy = GameBoy::new(rom);

    ctrlc::set_handler(move || {
        if is_debug_enabled() {
            // If already paused, stop the emulator
            println!("{}", "\nSo long space cowboy".red());
            exit(-1);
        } else {
            // If running, pause the emulator
            enable_debug();
            println!("Received Ctrl+C! Pausing at the end of this step...");
        }
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        // Enter debug mode if Ctrl-C received
        if is_debug_enabled() {
            gameboy.debugger_cli()
        }

        // Step forward
        gameboy.step();

        // Render window
        if gameboy.mmu.ppu.get_and_reset_frame_available() {
            let _ = tx.send(gameboy.mmu.ppu.clone());
        }

        // Handle joypad input
        loop {
            match rx.try_recv() {
                Ok((true, key)) => gameboy.mmu.joypad.handle_key_down(key),
                Ok((false, key)) => gameboy.mmu.joypad.handle_key_up(key),
                _ => break,
            }
        }
    }
}

fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
