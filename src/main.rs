pub mod cartridge;
pub mod cpu;
pub mod debugger;
pub mod gameboy;
pub mod instructions;
pub mod mmu;
mod renderer;

use cartridge::header::CartridgeHeader;
use clap::Parser;
use colored::*;
use debugger::{enable_debug, enable_gameboy_doctor, is_debug_enabled};
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

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to enable the doctor or not.
    #[arg(short, long, default_value_t = false)]
    doctor: bool,

    rom_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let rom_path = match args.rom_path {
        Some(path) => path,
        _ => "roms/super-mario-land.gb".to_string(),
    };

    if args.doctor {
        enable_gameboy_doctor();
    }

    let (tx, rx) = mpsc::channel::<PPU>();
    let (tx_key, rx_key) = mpsc::channel::<(bool, Key)>();
    let rom = read_file_to_bytes(rom_path.as_str()).unwrap();
    let game_title = CartridgeHeader::new(&rom).unwrap().title();

    let _ = thread::spawn(move || emulator_loop(rom, tx, rx_key));
    window_loop(rx, tx_key, &game_title);
}

fn emulator_loop(rom: Vec<u8>, tx: Sender<PPU>, rx: Receiver<(bool, Key)>) {
    let mut gameboy = GameBoy::new(rom);
    //gameboy.breakpoints.insert(0xC64E);
    //gameboy.breakpoints.insert(0xC642);

    ctrlc::set_handler(move || {
        if is_debug_enabled() {
            // If already paused, stop the emulator
            println!("{}", "\nSo long space cowboy".red());
            exit(-1);
        } else {
            // If running, pause the emulator
            enable_debug();
            println!("{}", "Received Ctrl+C! Entering debugger.".red());
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
