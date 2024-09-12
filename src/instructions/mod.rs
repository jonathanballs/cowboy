pub mod cond;
mod display;
pub mod r16;
pub mod r16mem;
pub mod r16stk;
pub mod r8;

use cond::Cond;
use r16::R16;
use r16mem::R16mem;
use r16stk::R16stk;
use r8::R8;

pub fn parse(opcode: u8, arg1: u8, arg2: u8) -> (Instruction, u16, u8) {
    let imm16: u16 = (arg2 as u16) << 8 | arg1 as u16;
    let imm8: u8 = arg1;

    return match opcode {
        // Block 0
        0x00 => (Instruction::Nop, 1, 4),
        0x01 => (Instruction::LdR16Imm16(R16::BC, imm16), 3, 12),
        0x02 => (Instruction::LdR16memA(R16mem::BC), 1, 8),
        0x03 => (Instruction::IncR16(R16::BC), 1, 8),
        0x04 => (Instruction::IncR8(R8::B), 1, 4),
        0x05 => (Instruction::DecR8(R8::B), 1, 4),
        0x06 => (Instruction::LdR8Imm8(R8::B, imm8), 2, 8),
        0x07 => (Instruction::Rlca, 1, 4),
        0x08 => (Instruction::LdImm16memSp(imm16), 3, 20),
        0x09 => (Instruction::AddHlR16(R16::BC), 1, 8),
        0x0A => (Instruction::LdAR16mem(R16mem::BC), 1, 8),
        0x0B => (Instruction::DecR16(R16::BC), 1, 8),
        0x0C => (Instruction::IncR8(R8::C), 1, 4),
        0x0D => (Instruction::DecR8(R8::C), 1, 4),
        0x0E => (Instruction::LdR8Imm8(R8::C, imm8), 2, 8),
        0x0F => (Instruction::Rrca, 1, 4),
        0x10 => (Instruction::Stop, 2, 4),
        0x11 => (Instruction::LdR16Imm16(R16::DE, imm16), 3, 12),
        0x12 => (Instruction::LdR16memA(R16mem::DE), 1, 8),
        0x13 => (Instruction::IncR16(R16::DE), 1, 8),
        0x14 => (Instruction::IncR8(R8::D), 1, 4),
        0x15 => (Instruction::DecR8(R8::D), 1, 4),
        0x16 => (Instruction::LdR8Imm8(R8::D, imm8), 2, 8),
        0x17 => (Instruction::Rla, 1, 4),
        0x18 => (Instruction::JrImm8(imm8 as i8), 2, 12),
        0x19 => (Instruction::AddHlR16(R16::DE), 1, 8),
        0x1A => (Instruction::LdAR16mem(R16mem::DE), 1, 8),
        0x1B => (Instruction::DecR16(R16::DE), 1, 8),
        0x1C => (Instruction::IncR8(R8::E), 1, 4),
        0x1D => (Instruction::DecR8(R8::E), 1, 4),
        0x1E => (Instruction::LdR8Imm8(R8::E, imm8), 2, 8),
        0x1F => (Instruction::Rra, 1, 4),
        0x20 => (Instruction::JrCondImm8(Cond::NZ, imm8 as i8), 2, 12),
        0x21 => (Instruction::LdR16Imm16(R16::HL, imm16), 3, 12),
        0x22 => (Instruction::LdR16memA(R16mem::HLI), 1, 8),
        0x23 => (Instruction::IncR16(R16::HL), 1, 8),
        0x24 => (Instruction::IncR8(R8::H), 1, 4),
        0x25 => (Instruction::DecR8(R8::H), 1, 4),
        0x26 => (Instruction::LdR8Imm8(R8::H, imm8), 2, 8),
        0x27 => (Instruction::Daa, 1, 4),
        0x28 => (Instruction::JrCondImm8(Cond::Z, imm8 as i8), 2, 12),
        0x29 => (Instruction::AddHlR16(R16::HL), 1, 8),
        0x2A => (Instruction::LdAR16mem(R16mem::HLI), 1, 8),
        0x2B => (Instruction::DecR16(R16::HL), 1, 8),
        0x2C => (Instruction::IncR8(R8::L), 1, 4),
        0x2D => (Instruction::DecR8(R8::L), 1, 4),
        0x2E => (Instruction::LdR8Imm8(R8::L, imm8), 2, 8),
        0x2F => (Instruction::Cpl, 1, 4),
        0x30 => (Instruction::JrCondImm8(Cond::NC, imm8 as i8), 2, 12),
        0x31 => (Instruction::LdR16Imm16(R16::SP, imm16), 3, 12),
        0x32 => (Instruction::LdR16memA(R16mem::HLD), 1, 8),
        0x33 => (Instruction::IncR16(R16::SP), 1, 8),
        0x34 => (Instruction::IncR8(R8::HL), 1, 12),
        0x35 => (Instruction::DecR8(R8::HL), 1, 12),
        0x36 => (Instruction::LdR8Imm8(R8::HL, imm8), 2, 12),
        0x37 => (Instruction::Scf, 1, 4),
        0x38 => (Instruction::JrCondImm8(Cond::C, imm8 as i8), 2, 12),
        0x39 => (Instruction::AddHlR16(R16::SP), 1, 8),
        0x3A => (Instruction::LdAR16mem(R16mem::HLD), 1, 8),
        0x3B => (Instruction::DecR16(R16::SP), 1, 8),
        0x3C => (Instruction::IncR8(R8::A), 1, 4),
        0x3D => (Instruction::DecR8(R8::A), 1, 4),
        0x3E => (Instruction::LdR8Imm8(R8::A, imm8), 2, 8),
        0x3F => (Instruction::Ccf, 1, 4),

        // Block 1
        0x76 => (Instruction::Halt, 1, 4),
        0x40..=0x7f => match (R8::from(opcode >> 3), R8::from(opcode)) {
            (R8::HL, R8::HL) => (Instruction::Halt, 1, 4),
            (R8::HL, source) => (Instruction::LdR8R8(R8::HL, source), 1, 8),
            (destin, R8::HL) => (Instruction::LdR8R8(destin, R8::HL), 1, 8),
            (destin, source) => (Instruction::LdR8R8(destin, source), 1, 4),
        },

        // Block 2
        0x80..=0xBF => {
            let operand = R8::from(opcode & 0x7);
            let cycle_count = match operand {
                R8::HL => 8,
                _ => 4,
            };

            return match (opcode >> 3) & 0x7 {
                0 => (Instruction::AddAR8(operand), 1, cycle_count),
                1 => (Instruction::AdcAR8(operand), 1, cycle_count),
                2 => (Instruction::SubAR8(operand), 1, cycle_count),
                3 => (Instruction::SbcAR8(operand), 1, cycle_count),
                4 => (Instruction::AndAR8(operand), 1, cycle_count),
                5 => (Instruction::XorAR8(operand), 1, cycle_count),
                6 => (Instruction::OrAR8(operand), 1, cycle_count),
                _ => (Instruction::CpAR8(operand), 1, cycle_count),
            };
        }

        // Block 3
        0xC0 => (Instruction::RetCond(Cond::NZ), 1, 20),
        0xC1 => (Instruction::PopR16stk(R16stk::BC), 1, 12),
        0xC2 => (Instruction::JpCondImm16(Cond::NZ, imm16), 3, 16),
        0xC3 => (Instruction::JpImm16(imm16), 3, 16),
        0xC4 => (Instruction::CallCondImm16(Cond::NZ, imm16), 3, 24),
        0xC5 => (Instruction::PushR16stk(R16stk::BC), 1, 16),
        0xC6 => (Instruction::AddAImm8(imm8), 2, 8),
        0xC7 => (Instruction::RstTgt3(0x00), 1, 16),
        0xC8 => (Instruction::RetCond(Cond::Z), 1, 20),
        0xC9 => (Instruction::Ret, 1, 16),
        0xCA => (Instruction::JpCondImm16(Cond::Z, imm16), 3, 16),
        0xCB => parse_prefixed(arg1),
        0xCC => (Instruction::CallCondImm16(Cond::Z, imm16), 3, 24),
        0xCD => (Instruction::CallImm16(imm16), 3, 24),
        0xCE => (Instruction::AdcAImm8(imm8), 2, 8),
        0xCF => (Instruction::RstTgt3(0x08), 1, 16),
        0xD0 => (Instruction::RetCond(Cond::NC), 1, 20),
        0xD1 => (Instruction::PopR16stk(R16stk::DE), 1, 12),
        0xD2 => (Instruction::JpCondImm16(Cond::NC, imm16), 3, 16),
        0xD4 => (Instruction::CallCondImm16(Cond::NC, imm16), 3, 24),
        0xD5 => (Instruction::PushR16stk(R16stk::DE), 1, 16),
        0xD6 => (Instruction::SubAImm8(imm8), 2, 8),
        0xD7 => (Instruction::RstTgt3(0x10), 1, 16),
        0xD8 => (Instruction::RetCond(Cond::C), 1, 20),
        0xD9 => (Instruction::Reti, 1, 16),
        0xDA => (Instruction::JpCondImm16(Cond::C, imm16), 3, 16),
        0xDC => (Instruction::CallCondImm16(Cond::C, imm16), 3, 24),
        0xDE => (Instruction::SbcAImm8(imm8), 2, 8),
        0xDF => (Instruction::RstTgt3(0x18), 1, 16),
        0xE0 => (Instruction::LdhImm8memA(imm8), 2, 12),
        0xE1 => (Instruction::PopR16stk(R16stk::HL), 1, 12),
        0xE2 => (Instruction::LdhCmemA, 1, 8),
        0xE5 => (Instruction::PushR16stk(R16stk::HL), 1, 16),
        0xE6 => (Instruction::AndAImm8(imm8), 2, 8),
        0xE7 => (Instruction::RstTgt3(0x20), 1, 16),
        0xE8 => (Instruction::AddSpImm8(imm8), 2, 16),
        0xE9 => (Instruction::JpHl, 1, 4),
        0xEA => (Instruction::LdImm16memA(imm16), 3, 16),
        0xEE => (Instruction::XorAImm8(imm8), 2, 8),
        0xEF => (Instruction::RstTgt3(0x28), 1, 16),
        0xF0 => (Instruction::LdhAImm8mem(imm8), 2, 12),
        0xF1 => (Instruction::PopR16stk(R16stk::AF), 1, 12),
        0xF2 => (Instruction::LdhACmem, 1, 8),
        0xF3 => (Instruction::Di, 1, 4),
        0xF5 => (Instruction::PushR16stk(R16stk::AF), 1, 16),
        0xF6 => (Instruction::OrAImm8(imm8), 2, 8),
        0xF7 => (Instruction::RstTgt3(0x30), 1, 16),
        0xF8 => (Instruction::LdHlSpImm8(imm8), 2, 12),
        0xF9 => (Instruction::LdSpHl, 1, 8),
        0xFA => (Instruction::LdAImm16mem(imm16), 3, 16),
        0xFB => (Instruction::Ei, 1, 4),
        0xFE => (Instruction::CpAImm8(imm8), 2, 8),
        0xFF => (Instruction::RstTgt3(0x38), 1, 16),

        // Illegal instructions
        0xD3 => (Instruction::ILLEGAL, 1, 4),
        0xDB => (Instruction::ILLEGAL, 1, 4),
        0xDD => (Instruction::ILLEGAL, 1, 4),
        0xE3 => (Instruction::ILLEGAL, 1, 4),
        0xE4 => (Instruction::ILLEGAL, 1, 4),
        0xEB => (Instruction::ILLEGAL, 1, 4),
        0xEC => (Instruction::ILLEGAL, 1, 4),
        0xED => (Instruction::ILLEGAL, 1, 4),
        0xF4 => (Instruction::ILLEGAL, 1, 4),
        0xFC => (Instruction::ILLEGAL, 1, 4),
        0xFD => (Instruction::ILLEGAL, 1, 4),
    };
}

fn parse_prefixed(opcode: u8) -> (Instruction, u16, u8) {
    let operand = R8::from(opcode & 0x7);

    let instruction = {
        if (opcode & 0xC0) == 0x0 {
            match (opcode & 0x38) >> 3 {
                0 => Instruction::RlcR8(operand),
                1 => Instruction::RrcR8(operand),
                2 => Instruction::RlR8(operand),
                3 => Instruction::RrR8(operand),
                4 => Instruction::SlaR8(operand),
                5 => Instruction::SraR8(operand),
                6 => Instruction::SwapR8(operand),
                _ => Instruction::SrlR8(operand),
            }
        } else {
            let bit_index = (opcode >> 3) & 0x7;

            match (opcode & 0xC0) >> 6 {
                0x1 => Instruction::BitB3R8(bit_index, operand),
                0x2 => Instruction::ResB3R8(bit_index, operand),
                0x3 => Instruction::SetB3R8(bit_index, operand),
                _ => unreachable!(),
            }
        }
    };

    return match instruction {
        Instruction::BitB3R8(_, R8::HL) => (instruction, 2, 12),
        Instruction::RlcR8(R8::HL) => (instruction, 2, 16),
        Instruction::RlR8(R8::HL) => (instruction, 2, 16),
        Instruction::SlaR8(R8::HL) => (instruction, 2, 16),
        Instruction::SwapR8(R8::HL) => (instruction, 2, 16),
        Instruction::RrcR8(R8::HL) => (instruction, 2, 16),
        Instruction::RrR8(R8::HL) => (instruction, 2, 16),
        Instruction::SraR8(R8::HL) => (instruction, 2, 16),
        Instruction::SrlR8(R8::HL) => (instruction, 2, 16),
        Instruction::ResB3R8(_, R8::HL) => (instruction, 2, 16),
        Instruction::SetB3R8(_, R8::HL) => (instruction, 2, 16),
        ins => (ins, 2, 8),
    };
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Stop,
    Halt,

    // Loading instructions
    LdAImm16mem(u16),
    LdAR16mem(R16mem),
    LdHlSpImm8(u8),
    LdImm16memA(u16),
    LdImm16memSp(u16),

    LdR16Imm16(R16, u16),
    LdR16memA(R16mem),

    LdR8Imm8(R8, u8),
    LdR8R8(R8, R8),
    LdSpHl,
    LdhACmem,
    LdhAImm8mem(u8),
    LdhCmemA,
    LdhImm8memA(u8),

    // Math instructions
    AdcAImm8(u8),
    AdcAR8(R8),
    AddAImm8(u8),
    AddAR8(R8),
    AddHlR16(R16),
    AddSpImm8(u8),
    AndAImm8(u8),
    AndAR8(R8),
    CpAImm8(u8),
    CpAR8(R8),
    DecR16(R16),
    DecR8(R8),
    IncR16(R16),
    IncR8(R8),
    OrAImm8(u8),
    OrAR8(R8),
    SbcAImm8(u8),
    SbcAR8(R8),
    SubAImm8(u8),
    SubAR8(R8),
    XorAImm8(u8),
    XorAR8(R8),

    // Rotation instructions
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    RlcR8(R8),
    RrcR8(R8),
    RlR8(R8),
    RrR8(R8),
    SlaR8(R8),
    SraR8(R8),
    SwapR8(R8),
    SrlR8(R8),

    // Stack instructions
    CallCondImm16(Cond, u16),
    CallImm16(u16),
    JpCondImm16(Cond, u16),
    JpHl,
    JpImm16(u16),
    JrCondImm8(Cond, i8),
    JrImm8(i8),
    PopR16stk(R16stk),
    PushR16stk(R16stk),
    Ret,
    RetCond(Cond),
    Reti,
    RstTgt3(u8),

    // Interrupts
    Di,
    Ei,

    // Bitwise Operations
    BitB3R8(u8, R8),
    ResB3R8(u8, R8),
    SetB3R8(u8, R8),

    ILLEGAL,
}

#[cfg(test)]
mod test {
    use serde_json::Value;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use super::parse;

    fn assert_instruction(opcode: u8, imm8: u8, json: &Value) {
        let expected_bytes = json["bytes"].as_u64().unwrap() as u16;
        let expected_cycles = json["cycles"][0].as_u64().unwrap() as u8;

        // Call the parse function (you need to implement this)
        let (_ins, actual_bytes, actual_cycles) = parse(opcode, imm8, 0x0);

        let error_message = format!(
            "{} (opcode: {:#04X} {:#04X})",
            json["mnemonic"].to_string(),
            opcode,
            imm8
        );

        assert_eq!(expected_bytes, actual_bytes, "{}", error_message,);
        assert_eq!(expected_cycles, actual_cycles, "{}", error_message,);
    }

    #[test]
    fn opcodes_json() {
        // Read the JSON file
        let filename = Path::new(file!()).parent().unwrap().join("opcodes.json");

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Parse the JSON
        let root: Value = serde_json::from_str(&contents).unwrap();

        // Iterate through each entry in the "unprefixed" object
        let unprefixed = root["unprefixed"]
            .as_object()
            .ok_or("Failed to find 'unprefixed' object in JSON")
            .unwrap();

        for (opcode, data) in unprefixed {
            // Parse the opcode string to a u8
            let opcode_value = u8::from_str_radix(&opcode[2..], 16).unwrap();
            if opcode_value == 0xCB {
                continue;
            }

            assert_instruction(opcode_value, 0x0, data);
        }

        // Iterate through each entry in the prefixed object
        let prefixed = root["cbprefixed"]
            .as_object()
            .ok_or("Failed to find 'prefixed' object in JSON")
            .unwrap();

        for (opcode, data) in prefixed {
            // Parse the opcode string to a u8
            let opcode_value = u8::from_str_radix(&opcode[2..], 16).unwrap();
            assert_instruction(0xCB, opcode_value, data);
        }
    }

    use crate::instructions::{Cond, Instruction, R16mem, R16, R8};

    fn get_instruction(opcode: u8, imm8: u8, arg2: u8) -> Instruction {
        match parse(opcode, imm8, arg2) {
            (ins, _, _) => ins,
        }
    }

    // Block 0
    #[test]
    fn nop() {
        assert!(matches!(get_instruction(0x0, 0x0, 0x0), Instruction::Nop))
    }

    #[test]
    fn jp_imm16() {
        // First instruction of Pokemon Gold :D
        assert!(matches!(
            get_instruction(0xC3, 0xC6, 0x5),
            Instruction::JpImm16(0x05C6)
        ))
    }

    #[test]
    fn ld_r16_imm16() {
        assert!(matches!(
            get_instruction(0x01, 0xC6, 0x5),
            Instruction::LdR16Imm16(R16::BC, 0x05C6)
        ));

        assert!(matches!(
            get_instruction(0x31, 0x0, 0x0),
            Instruction::LdR16Imm16(R16::SP, 0x0)
        ))
    }

    #[test]
    fn ld_r16mem_a() {
        assert!(matches!(
            get_instruction(0x02, 0x0, 0x0),
            Instruction::LdR16memA(R16mem::BC)
        ));
    }

    #[test]
    fn ld_a_r16mem() {
        assert!(matches!(
            get_instruction(0xA, 0x0, 0x0),
            Instruction::LdAR16mem(R16mem::BC)
        ));
    }

    #[test]
    fn ld_imm16_sp() {
        assert!(matches!(
            get_instruction(0x8, 0x34, 0x12),
            Instruction::LdImm16memSp(0x1234)
        ));
    }

    #[test]
    fn inc_r16() {
        assert!(matches!(
            get_instruction(0x3, 0x0, 0x0),
            Instruction::IncR16(R16::BC)
        ));
    }

    #[test]
    fn dec_r16() {
        assert!(matches!(
            get_instruction(0xB, 0x0, 0x0),
            Instruction::DecR16(R16::BC)
        ));
    }

    #[test]
    fn add_hl_r16() {
        assert!(matches!(
            get_instruction(0x9, 0x0, 0x0),
            Instruction::AddHlR16(R16::BC)
        ));
    }

    #[test]
    fn inc_r8() {
        assert!(matches!(
            get_instruction(0x4, 0x0, 0x0),
            Instruction::IncR8(R8::B)
        ));
    }

    #[test]
    fn dec_r8() {
        assert!(matches!(
            get_instruction(0x5, 0x0, 0x0),
            Instruction::DecR8(R8::B)
        ));
    }

    #[test]
    fn ld_r8_imm8() {
        assert!(matches!(
            get_instruction(0x6, 0x01, 0x02),
            Instruction::LdR8Imm8(R8::B, 0x1)
        ));
    }

    #[test]
    fn rlca() {
        assert!(matches!(
            get_instruction(0b111, 0x0, 0x0),
            Instruction::Rlca
        ))
    }

    #[test]
    fn rrca() {
        assert!(matches!(
            get_instruction(0b1111, 0x0, 0x0),
            Instruction::Rrca
        ))
    }

    #[test]
    fn rla() {
        assert!(matches!(
            get_instruction(0b10111, 0x0, 0x0),
            Instruction::Rla
        ))
    }

    #[test]
    fn rra() {
        assert!(matches!(
            get_instruction(0b11111, 0x0, 0x0),
            Instruction::Rra
        ))
    }

    #[test]
    fn daa() {
        assert!(matches!(
            get_instruction(0b100111, 0x0, 0x0),
            Instruction::Daa
        ))
    }

    #[test]
    fn cpl() {
        assert!(matches!(
            get_instruction(0b101111, 0x0, 0x0),
            Instruction::Cpl
        ))
    }

    #[test]
    fn scf() {
        assert!(matches!(
            get_instruction(0b110111, 0x0, 0x0),
            Instruction::Scf
        ))
    }

    #[test]
    fn ccf() {
        assert!(matches!(
            get_instruction(0b111111, 0x0, 0x0),
            Instruction::Ccf
        ))
    }

    #[test]
    fn jr_imm8() {
        assert!(matches!(
            get_instruction(0x18, 0xFF, 0x0),
            Instruction::JrImm8(-1)
        ))
    }

    #[test]
    fn jr_cond_imm8() {
        assert!(matches!(
            get_instruction(0x20, 0x20, 0x0),
            Instruction::JrCondImm8(Cond::NZ, 0x20)
        ))
    }

    #[test]
    fn stop() {
        assert!(matches!(
            get_instruction(0x10, 0x00, 0x0),
            Instruction::Stop
        ))
    }

    // block 1
    #[test]
    fn ld_r8_r8() {
        assert!(matches!(
            dbg!(get_instruction(0x61, 0x00, 0x0)),
            Instruction::LdR8R8(R8::H, R8::C)
        ))
    }

    #[test]
    fn halt() {
        assert!(matches!(
            get_instruction(0b01110110, 0x0, 0x0),
            Instruction::Halt
        ))
    }

    // Block 2
    #[test]
    fn add_a_r8() {
        assert!(matches!(
            get_instruction(0b10000000, 0x0, 0x0),
            Instruction::AddAR8(R8::B)
        ))
    }

    #[test]
    fn adc_a_r8() {
        assert!(matches!(
            get_instruction(0b10001000, 0x0, 0x0),
            Instruction::AdcAR8(R8::B)
        ))
    }

    #[test]
    fn sub_a_r8() {
        assert!(matches!(
            get_instruction(0b10010000, 0x0, 0x0),
            Instruction::SubAR8(R8::B)
        ))
    }

    #[test]
    fn sbc_a_r8() {
        assert!(matches!(
            get_instruction(0b10011000, 0x0, 0x0),
            Instruction::SbcAR8(R8::B)
        ))
    }

    #[test]
    fn and_a_r8() {
        assert!(matches!(
            get_instruction(0b10100000, 0x0, 0x0),
            Instruction::AndAR8(R8::B)
        ))
    }

    #[test]
    fn xor_a_r8() {
        assert!(matches!(
            get_instruction(0b10101000, 0x0, 0x0),
            Instruction::XorAR8(R8::B)
        ))
    }

    #[test]
    fn or_a_r8() {
        assert!(matches!(
            get_instruction(0b10110000, 0x0, 0x0),
            Instruction::OrAR8(R8::B)
        ))
    }

    #[test]
    fn cp_a_r8() {
        assert!(matches!(
            get_instruction(0b10111000, 0x0, 0x0),
            Instruction::CpAR8(R8::B)
        ))
    }

    // Block 3
    #[test]
    fn add_a_imm8() {
        assert!(matches!(
            get_instruction(0b11000110, 0x1, 0x0),
            Instruction::AddAImm8(0x1)
        ))
    }

    #[test]
    fn adc_a_imm8() {
        assert!(matches!(
            get_instruction(0b11001110, 0x1, 0x0),
            Instruction::AdcAImm8(0x1)
        ))
    }

    #[test]
    fn sub_a_imm8() {
        assert!(matches!(
            get_instruction(0b11010110, 0x1, 0x0),
            Instruction::SubAImm8(0x1)
        ))
    }

    #[test]
    fn sbc_a_imm8() {
        assert!(matches!(
            get_instruction(0b11011110, 0x1, 0x0),
            Instruction::SbcAImm8(0x1)
        ))
    }

    #[test]
    fn and_a_imm8() {
        assert!(matches!(
            get_instruction(0b11100110, 0x1, 0x0),
            Instruction::AndAImm8(0x1)
        ))
    }

    #[test]
    fn xor_a_imm8() {
        assert!(matches!(
            get_instruction(0b11101110, 0x1, 0x0),
            Instruction::XorAImm8(0x1)
        ))
    }

    #[test]
    fn or_a_imm8() {
        assert!(matches!(
            get_instruction(0b11110110, 0x1, 0x0),
            Instruction::OrAImm8(0x1)
        ))
    }

    #[test]
    fn cp_a_imm8() {
        assert!(matches!(
            get_instruction(0b11111110, 0x1, 0x0),
            Instruction::CpAImm8(0x1)
        ))
    }

    #[test]
    fn sub_a_r8a() {
        assert!(matches!(
            dbg!(get_instruction(0xD2, 0x0, 0x0)),
            Instruction::JpCondImm16(Cond::NC, _)
        ))
    }
}
