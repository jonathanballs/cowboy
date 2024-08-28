mod gameboy;
mod registers;
mod rom;

use std::fs::File;
use std::io::Read;

use gameboy::GameBoy;

fn read_file_to_bytes(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    match read_file_to_bytes("pokemon-gold.gbc") {
        Ok(rom_data) => {
            let gameboy = GameBoy::new(rom_data);
            println!("Read {:#?}", gameboy)
        }

        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
