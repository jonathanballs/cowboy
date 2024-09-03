use std::io::{self, Write};

use super::GameBoy;
use colored::*;

fn parse_number(s: &str) -> Result<u16, String> {
    if s.to_lowercase().starts_with("0x") {
        // Parse hexadecimal
        i64::from_str_radix(&s[2..], 16)
            .map(|n| n as u16)
            .map_err(|_| format!("Invalid hexadecimal number: {}", s))
    } else {
        // Parse decimal
        s.parse::<u16>()
            .map_err(|_| format!("Invalid decimal number: {}", s))
    }
}

impl GameBoy {
    pub fn format_instruction(&self) -> String {
        format!(
            //"{:#06X} {:#04X}: {} ({:X?})",
            "{:#06X} {:#04X}: {}",
            self.registers.pc,
            self.get_memory_byte(self.registers.pc),
            self.ins(),
            //self.ins(),
        )
    }

    fn print_memory_range(&self, start: u16, end: u16) {
        if start > end {
            println!("{}", "ERR: invalid byte range".red());
            return;
        }

        const BYTES_PER_ROW: u16 = 16;

        for offset in 0..(end - start) {
            if offset % BYTES_PER_ROW == 0 {
                print!("{:#06x}: ", start + offset);
            }

            print!("{:02x}", self.get_memory_byte(start + offset));

            if offset % 2 == 1 {
                print!(" ");
            }

            if offset % BYTES_PER_ROW == BYTES_PER_ROW - 1 {
                println!();
            }
        }

        println!();
    }

    pub fn debugger_cli(&mut self) {
        println!("{}", self.format_instruction());

        print!("{}", ">>> ".cyan());
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("").to_lowercase();
        let args: Vec<&str> = parts.collect();

        match command.as_str() {
            "s" | "step" => return,

            "c" | "continue" => {
                self.debugger_enabled = false;
                return;
            }

            "d" | "debug" => println!("{:#?}", &self),

            "p" | "m" | "mem" => {
                if args.len() != 2 {
                    println!("{}", "ERR: Please provide two numerical arguments".red());
                }

                match (parse_number(args[0]), parse_number(args[1])) {
                    (Ok(start), Ok(end)) => self.print_memory_range(start, end),
                    _ => {
                        println!("{}", "ERR: Invalid invalid numbers passed to mem".red());
                    }
                }
            }

            _ => {
                println!("{}", "ERR: Invalid debugger command".red());
            }
        }

        self.debugger_cli();
    }
}
