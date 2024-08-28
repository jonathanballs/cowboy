mod cond;
mod r16;
mod r16mem;
mod r8;

use core::fmt;

use cond::Cond;
use r16::R16;
use r16mem::R16mem;
use r8::R8;

pub fn parse(opcode: u8, arg1: u8, arg2: u8) -> Instruction {
    let imm16: u16 = (arg2 as u16) << 8 | arg1 as u16;
    let imm8: u8 = arg1;

    // Block 0
    if opcode == 0x00 {
        return Instruction::Nop;
    }

    if opcode == 0x10 {
        return Instruction::Stop;
    }

    if opcode & 0xCF == 0x01 {
        let dest = (opcode >> 4) & 0x03;
        return Instruction::LdR16Imm16(R16::from(dest), imm16);
    }

    if opcode & 0xCF == 0x02 {
        let dest = (opcode >> 4) & 0x03;
        return Instruction::LdR16memA(R16mem::from(dest));
    }

    if opcode & 0xCF == 0x0A {
        let src = (opcode >> 4) & 0x03;
        return Instruction::LdAR16mem(R16mem::from(src));
    }

    if opcode & 0xFF == 0x08 {
        return Instruction::LdImm16Sp(imm16);
    }

    if opcode & 0xCF == 0x03 {
        let operand = (opcode >> 4) & 0x03;
        return Instruction::IncR16(R16::from(operand));
    }

    if opcode & 0xCF == 0x0B {
        let operand = (opcode >> 4) & 0x03;
        return Instruction::DecR16(R16::from(operand));
    }

    if opcode & 0xCF == 0x09 {
        let operand = (opcode >> 4) & 0x03;
        return Instruction::AddHlR16(R16::from(operand));
    }

    if opcode & 0xC7 == 0x4 {
        let operand = (opcode >> 3) & 0x07;
        return Instruction::IncR8(R8::from(operand));
    }

    if opcode & 0xC7 == 0x5 {
        let operand = (opcode >> 3) & 0x07;
        return Instruction::DecR8(R8::from(operand));
    }

    if opcode & 0xC7 == 0x6 {
        let operand = (opcode >> 3) & 0x07;
        return Instruction::LdR8Imm8(R8::from(operand), imm8);
    }

    if opcode == 0x07 {
        return Instruction::Rlca;
    }

    if opcode == 0x0F {
        return Instruction::Rrca;
    }

    if opcode == 0x17 {
        return Instruction::Rla;
    }

    if opcode == 0x1F {
        return Instruction::Rra;
    }

    if opcode == 0x27 {
        return Instruction::Daa;
    }

    if opcode == 0x2F {
        return Instruction::Cpl;
    }

    if opcode == 0x37 {
        return Instruction::Scf;
    }

    if opcode == 0x3F {
        return Instruction::Ccf;
    }

    if opcode == 0x18 {
        return Instruction::JrImm8(imm8);
    }

    if (opcode & 0xE7) == 0x20 {
        return Instruction::JrCondImm8(Cond::from(opcode >> 3), imm8);
    }

    // Block 1
    if opcode == 0x76 {
        return Instruction::Halt;
    }

    if opcode & 0xC0 == 0x40 {
        let src = R8::from(opcode);
        let dest = R8::from(opcode >> 3);
        return Instruction::LdR8R8(dest, src);
    }

    // Block 2
    if opcode & 0xC0 == 0x80 {
        let operand = R8::from(opcode & 0x07);
        return match (opcode >> 3) & 0x7 {
            0 => Instruction::AddAR8(operand),
            1 => Instruction::AdcAR8(operand),
            2 => Instruction::SubAR8(operand),
            3 => Instruction::SbcAR8(operand),
            4 => Instruction::AndAR8(operand),
            5 => Instruction::XorAR8(operand),
            6 => Instruction::OrAR8(operand),
            _ => Instruction::CpAR8(operand),
        };
    }

    // Block 3
    assert!(opcode & 0xC0 == 0xC0);

    if opcode & 0x7 == 0x6 {
        return match opcode >> 3 {
            0 => Instruction::AddAImm8(imm8),
            1 => Instruction::AdcAImm8(imm8),
            2 => Instruction::SubAImm8(imm8),
            3 => Instruction::SbcAImm8(imm8),
            4 => Instruction::AndAImm8(imm8),
            5 => Instruction::XorAImm8(imm8),
            6 => Instruction::OrAImm8(imm8),
            _ => Instruction::CpAImm8(imm8),
        };
    }

    if opcode == 0xC3 {
        return Instruction::JpImm16(imm16);
    }

    // Block 3
    if opcode == 0xE9 {
        return Instruction::JpHl;
    }

    Instruction::ILLEGAL
}

#[derive(Debug)]
pub enum Instruction {
    // Block 0
    Nop,

    LdR16Imm16(R16, u16),
    LdR16memA(R16mem),
    LdAR16mem(R16mem),
    LdImm16Sp(u16),

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

    JpHl,
    JpImm16(u16),

    ILLEGAL,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Create a temporary buffer to capture the default debug output
        let mut buffer = String::new();

        // Use a temporary formatter to write the default debug format
        let _ = fmt::write(&mut buffer, format_args!("{:0x?}", self));

        let instruction_name = match buffer[1..].find(char::is_uppercase) {
            Some(index) => buffer[..index + 1].to_string(),
            None => buffer.clone(),
        };

        let args = match buffer.find('(') {
            Some(index) => &buffer[index + 1..buffer.len() - 1],
            None => "",
        };

        // Process the captured string (example: convert to uppercase)
        let processed = format!("{} {}", instruction_name.to_lowercase(), args);

        // Write the processed string to the actual formatter
        write!(f, "{}", processed)
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::{parse, Cond, Instruction, R16mem, R16, R8};

    // Block 0
    #[test]
    fn nop() {
        assert!(matches!(parse(0x0, 0x0, 0x0), Instruction::Nop))
    }

    #[test]
    fn jp_imm16() {
        // First instruction of Pokemon Gold :D
        assert!(matches!(
            parse(0xC3, 0xC6, 0x5),
            Instruction::JpImm16(0x05C6)
        ))
    }

    #[test]
    fn ld_r16_imm16() {
        assert!(matches!(
            parse(0x01, 0xC6, 0x5),
            Instruction::LdR16Imm16(R16::BC, 0x05C6)
        ));

        assert!(matches!(
            parse(0x31, 0x0, 0x0),
            Instruction::LdR16Imm16(R16::SP, 0x0)
        ))
    }

    #[test]
    fn ld_r16mem_a() {
        assert!(matches!(
            parse(0x02, 0x0, 0x0),
            Instruction::LdR16memA(R16mem::BC)
        ));
    }

    #[test]
    fn ld_a_r16mem() {
        assert!(matches!(
            parse(0xA, 0x0, 0x0),
            Instruction::LdAR16mem(R16mem::BC)
        ));
    }

    #[test]
    fn ld_imm16_sp() {
        assert!(matches!(
            parse(0x8, 0x34, 0x12),
            Instruction::LdImm16Sp(0x1234)
        ));
    }

    #[test]
    fn inc_r16() {
        assert!(matches!(parse(0x3, 0x0, 0x0), Instruction::IncR16(R16::BC)));
    }

    #[test]
    fn dec_r16() {
        assert!(matches!(parse(0xB, 0x0, 0x0), Instruction::DecR16(R16::BC)));
    }

    #[test]
    fn add_hl_r16() {
        assert!(matches!(
            parse(0x9, 0x0, 0x0),
            Instruction::AddHlR16(R16::BC)
        ));
    }

    #[test]
    fn inc_r8() {
        assert!(matches!(parse(0x4, 0x0, 0x0), Instruction::IncR8(R8::B)));
    }

    #[test]
    fn dec_r8() {
        assert!(matches!(parse(0x5, 0x0, 0x0), Instruction::DecR8(R8::B)));
    }

    #[test]
    fn ld_r8_imm8() {
        assert!(matches!(
            parse(0x6, 0x01, 0x02),
            Instruction::LdR8Imm8(R8::B, 0x1)
        ));
    }

    #[test]
    fn rlca() {
        assert!(matches!(parse(0b111, 0x0, 0x0), Instruction::Rlca))
    }

    #[test]
    fn rrca() {
        assert!(matches!(parse(0b1111, 0x0, 0x0), Instruction::Rrca))
    }

    #[test]
    fn rla() {
        assert!(matches!(parse(0b10111, 0x0, 0x0), Instruction::Rla))
    }

    #[test]
    fn rra() {
        assert!(matches!(parse(0b11111, 0x0, 0x0), Instruction::Rra))
    }

    #[test]
    fn daa() {
        assert!(matches!(parse(0b100111, 0x0, 0x0), Instruction::Daa))
    }

    #[test]
    fn cpl() {
        assert!(matches!(parse(0b101111, 0x0, 0x0), Instruction::Cpl))
    }

    #[test]
    fn scf() {
        assert!(matches!(parse(0b110111, 0x0, 0x0), Instruction::Scf))
    }

    #[test]
    fn ccf() {
        assert!(matches!(parse(0b111111, 0x0, 0x0), Instruction::Ccf))
    }

    #[test]
    fn jr_imm8() {
        assert!(matches!(parse(0x18, 0xFF, 0x0), Instruction::JrImm8(0xFF)))
    }

    #[test]
    fn jr_cond_imm8() {
        assert!(matches!(
            parse(0x20, 0x20, 0x0),
            Instruction::JrCondImm8(Cond::NZ, 0x20)
        ))
    }

    #[test]
    fn stop() {
        assert!(matches!(parse(0x10, 0x00, 0x0), Instruction::Stop))
    }

    // block 1
    #[test]
    fn ld_r8_r8() {
        assert!(matches!(
            parse(0x61, 0x00, 0x0),
            Instruction::LdR8R8(R8::H, R8::C)
        ))
    }

    #[test]
    fn halt() {
        assert!(matches!(parse(0b01110110, 0x0, 0x0), Instruction::Halt))
    }

    // Block 2
    #[test]
    fn add_a_r8() {
        assert!(matches!(
            parse(0b10000000, 0x0, 0x0),
            Instruction::AddAR8(R8::B)
        ))
    }

    #[test]
    fn sub_a_r8() {
        assert!(matches!(
            parse(0b10010000, 0x0, 0x0),
            Instruction::SubAR8(R8::B)
        ))
    }

    #[test]
    fn sbc_a_r8() {
        assert!(matches!(
            parse(0b10011000, 0x0, 0x0),
            Instruction::SbcAR8(R8::B)
        ))
    }

    #[test]
    fn and_a_r8() {
        assert!(matches!(
            parse(0b10100000, 0x0, 0x0),
            Instruction::AndAR8(R8::B)
        ))
    }

    #[test]
    fn xor_a_r8() {
        assert!(matches!(
            parse(0b10101000, 0x0, 0x0),
            Instruction::XorAR8(R8::B)
        ))
    }

    #[test]
    fn or_a_r8() {
        assert!(matches!(
            parse(0b10110000, 0x0, 0x0),
            Instruction::OrAR8(R8::B)
        ))
    }

    #[test]
    fn cp_a_r8() {
        assert!(matches!(
            parse(0b10111000, 0x0, 0x0),
            Instruction::CpAR8(R8::B)
        ))
    }
}
