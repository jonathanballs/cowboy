use core::fmt;
use std::io::{self, Write};

use crate::{
    debugger::{disable_debug, enable_debug},
    instructions::parse,
};

use super::GameBoy;
use colored::*;

fn parse_number(s: &str) -> Option<u16> {
    if s.to_lowercase().starts_with("0x") {
        u16::from_str_radix(&s[2..], 16).ok()
    } else if s.to_lowercase().starts_with("0b") {
        u16::from_str_radix(&s[2..], 2).ok()
    } else {
        u16::from_str_radix(s, 10).ok()
    }
}

impl GameBoy {
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
                    enable_debug();
                    return;
                }

                "c" | "continue" => {
                    disable_debug();
                    return;
                }

                "d" | "debug" => println!("{:#?}", &self),

                "ppu" => println!("{:#?}", &self.mmu.ppu),

                "f" | "flush" => {
                    //pub fn flush(&self) {
                    //    self.tx.send(self.clone()).unwrap();
                    //}
                    //self.ppu.flush();
                    println!("UNSUPPORTED")
                }

                "ro" | "rom" => println!("{:#?}", self.mmu.cartridge.header),

                "r" | "registers" => println!("{:#?}", self.cpu.registers),

                "h" | "help" => self.print_help(),

                "i" | "interrupts" => self.print_interrupts(),

                "ins" | "instructions" => self.print_instructions(),

                "p" | "print" => self.print_memory_range(args),

                "b" | "break" => {
                    if args.len() != 1 {
                        println!("{:?}", self.breakpoints);
                        println!("{}", "ERR: Please provide two numerical arguments".red());
                        continue;
                    }

                    match parse_number(args[0]) {
                        Some(breakpoint) => {
                            if self.breakpoints.contains(&breakpoint) {
                                println!("{}", "Removing breakpoint".green());
                                self.breakpoints.remove(&breakpoint);
                            } else {
                                self.breakpoints.insert(breakpoint);
                            }
                        }
                        _ => {
                            println!("{}", "ERR: Invalid invalid numbers passed to break".red());
                        }
                    }
                }

                "bm" | "breakmemory" => {
                    if args.len() != 1 {
                        println!("{}", "ERR: Please provide two numerical arguments".red());
                        continue;
                    }

                    match parse_number(args[0]) {
                        Some(breakpoint) => {
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

    pub fn format_instruction(&self) -> String {
        let instruction_address = format!("{:#06X}", self.cpu.registers.pc);

        let opcode = self.mmu.read_byte(self.cpu.registers.pc);
        let arg_1 = self.mmu.read_byte(self.cpu.registers.pc + 1);
        let arg_2 = self.mmu.read_byte(self.cpu.registers.pc + 2);

        let (_, instruction_length, _) = parse(opcode, arg_1, arg_2);
        let instruction_bytes = (0..instruction_length)
            .map(|o| {
                format!(
                    "{:02x}",
                    self.mmu.read_byte(self.cpu.registers.pc + o as u16)
                )
            })
            .collect::<Vec<String>>()
            .join("");

        format!(
            "{} 0x{}: {}",
            instruction_address,
            instruction_bytes,
            self.ins(),
        )
    }

    fn print_memory_range(&self, args: Vec<&str>) {
        const BYTES_PER_ROW: u16 = 16;

        let parsed_args = args.iter().map(|s| parse_number(s)).fold(
            Some(vec![] as Vec<u16>),
            |acc, curr| match (acc, curr) {
                (None, _) => None,
                (_, None) => None,
                (Some(mut v), Some(n)) => {
                    v.push(n);
                    Some(v.to_vec())
                }
            },
        );

        let (start, end) = match parsed_args {
            Some(nums) => match nums.as_slice() {
                [start] => (*start, *start + 0xF),
                [start, end] => (*start, *end),
                _ => return,
            },
            None => return,
        };

        if start > end {
            println!("{}", "ERR: start must be before end".red());
            return;
        }

        for offset in 0..(end - start) {
            if offset % BYTES_PER_ROW == 0 {
                print!("{:#06x}: ", start + offset);
            }

            print!("{:02x}", self.mmu.read_byte(start + offset));

            if offset % 2 == 1 {
                print!(" ");
            }

            if offset % BYTES_PER_ROW == BYTES_PER_ROW - 1 {
                println!();
            }
        }

        println!();
    }

    fn print_instructions(&self) {
        for (addr, instruction) in self.instruction_history.iter() {
            println!("{:#06X} {}", addr, instruction)
        }
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
        println!("IME:       {}", colored_bool(self.cpu.ime));
        println!("");
        println!(
            "VBlank:    {:5}     {}",
            colored_bool(self.mmu.ie & 0x1 > 0),
            colored_bool(self.mmu.read_byte(0xFF0F) & 0x1 > 0)
        );
        println!(
            "LCD:       {:5}     {}",
            colored_bool(self.mmu.ie & 0x2 > 0),
            colored_bool(self.mmu.read_byte(0xFF0F) & 0x2 > 0)
        );
        println!(
            "Timer:     {:5}     {}",
            colored_bool(self.mmu.ie & 0x4 > 0),
            colored_bool(self.mmu.read_byte(0xFF0F) & 0x4 > 0)
        );
        println!(
            "Serial:    {:5}     {}",
            colored_bool(self.mmu.ie & 0x8 > 0),
            colored_bool(self.mmu.read_byte(0xFF0F) & 0x8 > 0)
        );
        println!(
            "Joypad:    {:5}     {}",
            colored_bool(self.mmu.ie & 0x10 > 0),
            colored_bool(self.mmu.read_byte(0xFF0F) & 0x10 > 0)
        );

        println!("");
    }

    fn print_help(&self) {
        println!("============== COWBOY DEBUGGER ==============");
        println!("[s]tep | <Enter>          step an instruction");
        println!("[c]ontinue                leave debug session");
        println!("[p]rint a b               dump gameboy memory");
        println!("[b]reak a                 breakpoint creation");
        println!("[b]reak [m]emory a        break on mem access");
        println!("[d]ebug                   print gameboy state");
        println!("[f]lush                   flush ppu to screen");
        println!("[r]egisters               print cpu registers");
        println!("[i]nterrupts              show interupt flags");
        println!("[h]elp                    show this help info");
        println!("[ro]m                     display gameboy rom");
        println!("[ins]tructions            last cpu operations");
        println!("=============================================");
        println!("");
    }
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBoy")
            .field("cpu", &self.cpu)
            .field("ie", &self.mmu.ie)
            //.field("instruction", &self.ins())
            .field(
                "instruction_raw",
                &self.mmu.read_byte(self.cpu.registers.pc),
            )
            .finish()
    }
}
