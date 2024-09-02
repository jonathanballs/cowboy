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

pub fn parse(opcode: u8, arg1: u8, arg2: u8) -> (Instruction, u8, u8) {
    let imm16: u16 = (arg2 as u16) << 8 | arg1 as u16;
    let imm8: u8 = arg1;

    return match opcode {
        // Block 0
        0x00 => (Instruction::Nop, 1, 4),
        0x01 => (Instruction::LdR16Imm16mem(R16::BC, imm16), 3, 12),
        0x02 => (Instruction::LdR16memA(R16mem::BC), 1, 8),
        0x03 => (Instruction::IncR16(R16::BC), 1, 8),
        0x04 => (Instruction::IncR8(R8::B), 1, 4),
        0x05 => (Instruction::IncR8(R8::B), 1, 4),
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
        0x11 => (Instruction::LdR16Imm16mem(R16::DE, imm16), 3, 12),
        0x12 => (Instruction::LdR16memA(R16mem::DE), 1, 8),
        0x13 => (Instruction::IncR16(R16::DE), 1, 8),
        0x14 => (Instruction::IncR8(R8::D), 1, 4),
        0x15 => (Instruction::IncR8(R8::D), 1, 4),
        0x16 => (Instruction::LdR8Imm8(R8::D, imm8), 2, 8),
        0x17 => (Instruction::Rla, 1, 4),
        0x18 => (Instruction::JrImm8(imm8), 2, 12),
        0x19 => (Instruction::AddHlR16(R16::DE), 1, 8),
        0x1A => (Instruction::LdAR16mem(R16mem::DE), 1, 8),
        0x1B => (Instruction::DecR16(R16::DE), 1, 8),
        0x1C => (Instruction::IncR8(R8::E), 1, 4),
        0x1D => (Instruction::DecR8(R8::E), 1, 4),
        0x1E => (Instruction::LdR8Imm8(R8::E, imm8), 2, 8),
        0x1F => (Instruction::Rra, 1, 4),
        0x20 => (Instruction::JrCondImm8(Cond::NZ, imm8), 2, 8),
        0x21 => (Instruction::LdR16Imm16mem(R16::HL, imm16), 3, 12),
        0x22 => (Instruction::LdR16memA(R16mem::HLI), 1, 8),
        0x23 => (Instruction::IncR16(R16::HL), 1, 8),
        0x24 => (Instruction::IncR8(R8::H), 1, 4),
        0x25 => (Instruction::IncR8(R8::H), 1, 4),
        0x26 => (Instruction::LdR8Imm8(R8::H, imm8), 2, 8),
        0x27 => (Instruction::Daa, 1, 4),
        0x28 => (Instruction::JrCondImm8(Cond::Z, imm8), 2, 8),
        0x29 => (Instruction::AddHlR16(R16::HL), 1, 8),
        0x2A => (Instruction::LdAR16mem(R16mem::HLI), 1, 8),
        0x2B => (Instruction::DecR16(R16::HL), 1, 8),
        0x2C => (Instruction::IncR8(R8::L), 1, 4),
        0x2D => (Instruction::DecR8(R8::L), 1, 4),
        0x2E => (Instruction::LdR8Imm8(R8::L, imm8), 2, 8),
        0x30 => (Instruction::JrCondImm8(Cond::NC, imm8), 2, 8),
        0x31 => (Instruction::LdR16Imm16mem(R16::SP, imm16), 3, 12),
        0x32 => (Instruction::LdR16memA(R16mem::HLD), 1, 8),
        0x33 => (Instruction::IncR16(R16::SP), 1, 8),
        0x34 => (Instruction::IncR8(R8::HL), 1, 12),
        0x35 => (Instruction::DecR8(R8::HL), 1, 12),
        0x36 => (Instruction::LdR8Imm8(R8::HL, imm8), 2, 8),
        0x37 => (Instruction::Scf, 1, 4),
        0x38 => (Instruction::JrCondImm8(Cond::C, imm8), 2, 8),
        0x39 => (Instruction::AddHlR16(R16::SP), 1, 8),
        0x3A => (Instruction::LdAR16mem(R16mem::HLD), 1, 8),
        0x3B => (Instruction::DecR16(R16::SP), 1, 8),
        0x3C => (Instruction::IncR8(R8::A), 1, 4),
        0x3D => (Instruction::DecR8(R8::A), 1, 4),
        0x3E => (Instruction::LdR8Imm8(R8::A, imm8), 2, 8),

        // Block 1
        0x76 => (Instruction::Halt, 1, 4),
        0x40..=0x7f => match (R8::from(opcode), R8::from(opcode >> 3)) {
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
        _ => unreachable!(),
    };

    //// Block 3
    //assert!(opcode & 0xC0 == 0xC0);
    //
    //if opcode & 0x7 == 0x6 {
    //    return match (opcode >> 3) & 0x7 {
    //        0 => Instruction::AddAImm8(imm8),
    //        1 => Instruction::AdcAImm8(imm8),
    //        2 => Instruction::SubAImm8(imm8),
    //        3 => Instruction::SbcAImm8(imm8),
    //        4 => Instruction::AndAImm8(imm8),
    //        5 => Instruction::XorAImm8(imm8),
    //        6 => Instruction::OrAImm8(imm8),
    //        _ => Instruction::CpAImm8(imm8),
    //    };
    //}
    //
    //if opcode & 0xE7 == 0xC0 {
    //    return Instruction::RetCond(Cond::from((opcode >> 3) & 0x3));
    //}
    //
    //if opcode == 0xC9 {
    //    return Instruction::Ret;
    //}
    //
    //if opcode == 0xD9 {
    //    return Instruction::Reti;
    //}
    //
    //if opcode & 0xE7 == 0xC2 {
    //    return Instruction::JpCondImm16(Cond::from((opcode >> 3) & 0x3), imm16);
    //}
    //
    //if opcode == 0xC3 {
    //    return Instruction::JpImm16(imm16);
    //}
    //
    //if opcode == 0xE9 {
    //    return Instruction::JpHl;
    //}
    //
    //if opcode & 0xE7 == 0xC4 {
    //    return Instruction::CallCondImm16(Cond::from((opcode >> 3) & 0x3), imm16);
    //}
    //
    //if opcode == 0xCD {
    //    return Instruction::CallImm16(imm16);
    //}
    //
    //if opcode & 0xC7 == 0xC7 {
    //    return Instruction::RstTgt3((opcode >> 3) & 0x7);
    //}
    //
    //if opcode & 0xCF == 0xC1 {
    //    return Instruction::PopR16stk(R16stk::from(opcode >> 3));
    //}
    //
    //if opcode & 0xCF == 0xC5 {
    //    return Instruction::PushR16stk(R16stk::from(opcode >> 3));
    //}
    //
    //if opcode == 0xE2 {
    //    return Instruction::LdhCmemA;
    //}
    //
    //if opcode == 0xE0 {
    //    return Instruction::LdhImm8memA(imm8);
    //}
    //
    //if opcode == 0xEA {
    //    return Instruction::LdImm16memA(imm16);
    //}
    //
    //if opcode == 0xF2 {
    //    return Instruction::LdhACmem;
    //}
    //
    //if opcode == 0xF0 {
    //    return Instruction::LdhAImm8mem(imm8);
    //}
    //
    //if opcode == 0xFA {
    //    return Instruction::LdAImm16mem(imm16);
    //}
    //
    //if opcode == 0xE8 {
    //    return Instruction::AddSpImm8(imm8);
    //}
    //
    //if opcode == 0xF8 {
    //    return Instruction::LdHlSpImm8(imm8);
    //}
    //
    //if opcode == 0xF9 {
    //    return Instruction::LdSpHl;
    //}
    //
    //if opcode == 0xF3 {
    //    return Instruction::Di;
    //}
    //
    //if opcode == 0xFB {
    //    return Instruction::Ei;
    //}
    //
    //// Prefixed instructions
    //if opcode == 0xCB {
    //    let operand = R8::from(arg1 & 0x7);
    //
    //    if (arg1 & 0xC0) == 0x0 {
    //        return match (arg1 & 0x38) >> 3 {
    //            0 => Instruction::RlcR8(operand),
    //            1 => Instruction::RrcR8(operand),
    //            2 => Instruction::RlR8(operand),
    //            3 => Instruction::RrR8(operand),
    //            4 => Instruction::SlaR8(operand),
    //            5 => Instruction::SraR8(operand),
    //            6 => Instruction::SwapR8(operand),
    //            _ => Instruction::SrlR8(operand),
    //        };
    //    }
    //
    //    let bit_index = (arg1 >> 3) & 0x7;
    //
    //    return match (arg1 & 0xC0) >> 6 {
    //        0x1 => Instruction::BitB3R8(bit_index, operand),
    //        0x2 => Instruction::ResB3R8(bit_index, operand),
    //        0x3 => Instruction::SetB3R8(bit_index, operand),
    //        _ => unreachable!(),
    //    };
    //}
    //
    //Instruction::ILLEGAL
}

#[derive(Debug)]
pub enum Instruction {
    // Block 0
    Nop,

    LdR16Imm16mem(R16, u16),
    LdR16memA(R16mem),
    LdAR16mem(R16mem),
    LdImm16memSp(u16),

    IncR16(R16),
    DecR16(R16),
    AddHlR16(R16),

    IncR8(R8),
    DecR8(R8),

    LdR8Imm8(R8, u8),

    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,

    JrImm8(u8),
    JrCondImm8(Cond, u8),

    Stop,

    // Block 1
    Halt,
    LdR8R8(R8, R8),

    // Block 2
    AddAR8(R8),
    AdcAR8(R8),
    SubAR8(R8),
    SbcAR8(R8),
    AndAR8(R8),
    XorAR8(R8),
    OrAR8(R8),
    CpAR8(R8),

    // Block 3
    AddAImm8(u8),
    AdcAImm8(u8),
    SubAImm8(u8),
    SbcAImm8(u8),
    AndAImm8(u8),
    XorAImm8(u8),
    OrAImm8(u8),
    CpAImm8(u8),

    RetCond(Cond),
    Ret,
    Reti,
    JpCondImm16(Cond, u16),
    JpImm16(u16),
    JpHl,
    CallCondImm16(Cond, u16),
    CallImm16(u16),
    RstTgt3(u8),
    PopR16stk(R16stk),
    PushR16stk(R16stk),

    LdhCmemA,
    LdhImm8memA(u8),
    LdImm16memA(u16),
    LdhACmem,
    LdhAImm8mem(u8),
    LdAImm16mem(u16),

    AddSpImm8(u8),
    LdHlSpImm8(u8),
    LdSpHl,
    Di,
    Ei,

    // Prefix
    RlcR8(R8),
    RrcR8(R8),
    RlR8(R8),
    RrR8(R8),
    SlaR8(R8),
    SraR8(R8),
    SwapR8(R8),
    SrlR8(R8),

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

    #[test]
    fn opcodes_json() {
        // Read the JSON file
        let filename = Path::new(file!())
            .parent()
            .unwrap()
            .join("instructions/opcodes.json");

        println!("{}", filename.to_string_lossy());

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Parse the JSON
        let root: Value = serde_json::from_str(&contents).unwrap();

        // Iterate through each entry in the "unprefixed" object
        //
        let unprefixed = root["unprefixed"]
            .as_object()
            .ok_or("Failed to find 'unprefixed' object in JSON")
            .unwrap();

        for (opcode, data) in unprefixed {
            // Parse the opcode string to a u8
            let opcode_value = u8::from_str_radix(&opcode[2..], 16).unwrap();

            // Get the expected values
            let expected_bytes = data["bytes"].as_u64().unwrap() as u8;
            let expected_cycles = data["cycles"][0].as_u64().unwrap() as u8;

            // Call the parse function (you need to implement this)
            let (_ins, actual_bytes, actual_cycles) = parse(opcode_value, 0x0, 0x0);

            assert_eq!(
                expected_bytes,
                actual_bytes,
                "{} (opcode: {:#04x})",
                data["mnemonic"].to_string(),
                opcode_value
            );

            assert_eq!(
                expected_cycles,
                actual_cycles,
                "{} (opcode: {:#04x})",
                data["mnemonic"].to_string(),
                opcode_value
            );
        }
    }
}
