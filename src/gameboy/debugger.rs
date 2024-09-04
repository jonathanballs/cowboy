use core::fmt;
use std::io::{self, Write};

use crate::{instructions::parse, rom::GBCHeader};

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
        let instruction_address = format!("{:#06X}", self.registers.pc);

        let opcode = self.get_memory_byte(self.registers.pc);
        let arg_1 = self.get_memory_byte(self.registers.pc + 1);
        let arg_2 = self.get_memory_byte(self.registers.pc + 2);

        let (_, instruction_length, _) = parse(opcode, arg_1, arg_2);
        let instruction_bytes = (0..instruction_length)
            .map(|o| format!("{:x}", self.get_memory_byte(self.registers.pc + o as u16)))
            .collect::<Vec<String>>()
            .join("");

        format!(
            "{} 0x{}: {}",
            instruction_address,
            instruction_bytes,
            self.ins(),
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

    fn print_interrupts(&self) {
        fn colored_bool(b: bool) -> String {
            if b {
                return "true ".green().to_string();
            }
            return "false".red().to_string();
        }

        println!("======== interrupts ========");
        println!("           enabled   flagged");
        println!("IME:       {}", colored_bool(self.ime));
        println!("");
        println!(
            "VBlank:    {:5}     {}",
            colored_bool(self.ie & 0x1 > 0),
            colored_bool(self.get_memory_byte(0xFF0F) & 0x1 > 0)
        );
        println!(
            "LCD:       {:5}     {}",
            colored_bool(self.ie & 0x2 > 0),
            colored_bool(self.get_memory_byte(0xFF0F) & 0x2 > 0)
        );
        println!(
            "Timer:     {:5}     {}",
            colored_bool(self.ie & 0x4 > 0),
            colored_bool(self.get_memory_byte(0xFF0F) & 0x4 > 0)
        );
        println!(
            "Serial:    {:5}     {}",
            colored_bool(self.ie & 0x8 > 0),
            colored_bool(self.get_memory_byte(0xFF0F) & 0x8 > 0)
        );
        println!(
            "Joypad:    {:5}     {}",
            colored_bool(self.ie & 0x16 > 0),
            colored_bool(self.get_memory_byte(0xFF0F) & 0x16 > 0)
        );

        println!("");
    }

    pub fn debugger_cli(&mut self) {
        println!("{}", self.format_instruction());
        loop {
            print!("{}", ">>> ".cyan());
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();
            let mut parts = input.split_whitespace();
            let command = parts.next().unwrap_or("").to_lowercase();
            let args: Vec<&str> = parts.collect();

            match command.as_str() {
                "" | "s" | "step" => {
                    self.debugger_enabled = true;
                    return;
                }

                "c" | "continue" => {
                    self.debugger_enabled = false;
                    return;
                }

                "d" | "debug" => println!("{:#?}", &self),

                "f" | "flush" => {
                    self.ppu.flush();
                    println!("OK")
                }

                "ro" | "rom" => println!("{:#?}", &GBCHeader::new(&self.rom_data)),

                "r" | "registers" => println!("{:#?}", self.registers),

                "h" | "help" => self.print_help(),

                "i" | "interrupts" => self.print_interrupts(),

                "p" | "print" => {
                    if args.len() != 2 {
                        println!("{}", "ERR: Please provide two numerical arguments".red());
                        continue;
                    }

                    match (parse_number(args[0]), parse_number(args[1])) {
                        (Ok(start), Ok(end)) => self.print_memory_range(start, end),
                        _ => {
                            println!("{}", "ERR: Invalid invalid numbers passed to mem".red());
                        }
                    }
                }

                "b" | "break" => {
                    if args.len() != 1 {
                        println!("{}", "ERR: Please provide two numerical arguments".red());
                        continue;
                    }

                    match parse_number(args[0]) {
                        Ok(breakpoint) => {
                            self.breakpoints.insert(breakpoint);
                        }
                        _ => {
                            println!("{}", "ERR: Invalid invalid numbers passed to mem".red());
                        }
                    }
                }

                "bm" | "breakmemory" => {
                    if args.len() != 1 {
                        println!("{}", "ERR: Please provide two numerical arguments".red());
                        continue;
                    }

                    match parse_number(args[0]) {
                        Ok(breakpoint) => {
                            self.memory_breakpoints.insert(breakpoint);
                        }
                        _ => {
                            println!("{}", "ERR: Invalid invalid numbers passed to mem".red());
                        }
                    }
                }

                _ => {
                    println!("{}", "ERR: Invalid debugger command".red());
                }
            }
        }
    }

    fn print_help(&self) {
        println!("============== COWBOY DEBUGGER ==============");
        println!("[s]tep | <Enter>          step an instruction");
        println!("[p]rint a b               dump gameboy memory");
        println!("[b]reak a                 breakpoint creation");
        println!("[b]reak [m]emory a        break on mem access");
        println!("[d]ebug                   print gameboy state");
        println!("[f]lush                   flush ppu to screen");
        println!("[r]egisters               print cpu registers");
        println!("[i]nterrupts              show interupt flags");
        println!("[h]elp                    show this help info");
        println!("[ro]m                     display gameboy rom");
        println!("=============================================");
        println!("");
    }
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("ime", &self.ime)
            .field("ie", &self.ie)
            .field("div", &self.div)
            .field("tima", &self.tima)
            .field("tma", &self.tma)
            .field("tac", &self.tac)
            .field("joypad", &self.joypad)
            .field("dulr", &self.dulr)
            .field("ssba", &self.ssba)
            .field("instruction", &self.ins())
            .field("instruction_raw", &self.get_memory_byte(self.registers.pc))
            .finish()
    }
}
