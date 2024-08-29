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
        let operand = R8::from(opcode & 0x7);
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
        return match (opcode >> 3) & 0x7 {
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

    if opcode & 0xE7 == 0xC0 {
        return Instruction::RetCond(Cond::from((opcode >> 3) & 0x3));
    }

    if opcode == 0xC9 {
        return Instruction::Ret;
    }

    if opcode == 0xD9 {
        return Instruction::Reti;
    }

    if opcode & 0xE7 == 0xC2 {
        return Instruction::JpCondImm16(Cond::from((opcode >> 3) & 0x3), imm16);
    }

    if opcode == 0xC3 {
        return Instruction::JpImm16(imm16);
    }

    if opcode == 0xE9 {
        return Instruction::JpHl;
    }

    if opcode & 0xE7 == 0xC4 {
        return Instruction::CallCondImm16(Cond::from((opcode >> 3) & 0x3), imm16);
    }

    if opcode == 0xCD {
        return Instruction::CallImm16(imm16);
    }

    if opcode & 0xC7 == 0xC7 {
        return Instruction::RstTgt3((opcode >> 3) & 0x7);
    }

    if opcode & 0xCF == 0xC1 {
        return Instruction::PopR16stk(R16stk::from(opcode >> 3));
    }

    if opcode & 0xCF == 0xC5 {
        return Instruction::PushR16stk(R16stk::from(opcode >> 3));
    }

    if opcode == 0xE2 {
        return Instruction::LdhCA;
    }

    if opcode == 0xE0 {
        return Instruction::LdhImm8A(imm8);
    }

    if opcode == 0xEA {
        return Instruction::LdImm16A(imm16);
    }

    if opcode == 0xF2 {
        return Instruction::LdhAC;
    }

    if opcode == 0xF0 {
        return Instruction::LdhAImm8(imm8);
    }

    if opcode == 0xFA {
        return Instruction::LdAImm16(imm16);
    }

    if opcode == 0xE8 {
        return Instruction::AddSpImm8(imm8);
    }

    if opcode == 0xF8 {
        return Instruction::LdHlSpImm8(imm8);
    }

    if opcode == 0xF9 {
        return Instruction::LdSpHl;
    }

    if opcode == 0xF3 {
        return Instruction::Di;
    }

    if opcode == 0xFB {
        return Instruction::Ei;
    }

    // Prefixed instructions
    if opcode == 0xCB {
        let operand = R8::from(arg1 & 0x7);

        if (arg1 & 0xC0) == 0x0 {
            return match arg1 & 0x38 {
                0 => Instruction::RlcR8(operand),
                1 => Instruction::RrcR8(operand),
                2 => Instruction::RlR8(operand),
                3 => Instruction::RrR8(operand),
                4 => Instruction::SlaR8(operand),
                5 => Instruction::SraR8(operand),
                6 => Instruction::SwapR8(operand),
                _ => Instruction::SrlR8(operand),
            };
        }

        let bit_index = (arg1 >> 3) & 0x7;

        return match (arg1 & 0xC0) >> 6 {
            0x1 => Instruction::BitB3R8(bit_index, operand),
            0x2 => Instruction::ResB3R8(bit_index, operand),
            0x3 => Instruction::SetB3R8(bit_index, operand),
            _ => unreachable!(),
        };
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

    LdhCA,
    LdhImm8A(u8),
    LdImm16A(u16),
    LdhAC,
    LdhAImm8(u8),
    LdAImm16(u16),

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
    fn adc_a_r8() {
        assert!(matches!(
            parse(0b10001000, 0x0, 0x0),
            Instruction::AdcAR8(R8::B)
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

    // Block 3
    #[test]
    fn add_a_imm8() {
        assert!(matches!(
            parse(0b11000110, 0x1, 0x0),
            Instruction::AddAImm8(0x1)
        ))
    }

    #[test]
    fn adc_a_imm8() {
        assert!(matches!(
            parse(0b11001110, 0x1, 0x0),
            Instruction::AdcAImm8(0x1)
        ))
    }

    #[test]
    fn sub_a_imm8() {
        assert!(matches!(
            parse(0b11010110, 0x1, 0x0),
            Instruction::SubAImm8(0x1)
        ))
    }

    #[test]
    fn sbc_a_imm8() {
        assert!(matches!(
            parse(0b11011110, 0x1, 0x0),
            Instruction::SbcAImm8(0x1)
        ))
    }

    #[test]
    fn and_a_imm8() {
        assert!(matches!(
            parse(0b11100110, 0x1, 0x0),
            Instruction::AndAImm8(0x1)
        ))
    }

    #[test]
    fn xor_a_imm8() {
        assert!(matches!(
            parse(0b11101110, 0x1, 0x0),
            Instruction::XorAImm8(0x1)
        ))
    }

    #[test]
    fn or_a_imm8() {
        assert!(matches!(
            parse(0b11110110, 0x1, 0x0),
            Instruction::OrAImm8(0x1)
        ))
    }

    #[test]
    fn cp_a_imm8() {
        assert!(matches!(
            parse(0b11111110, 0x1, 0x0),
            Instruction::CpAImm8(0x1)
        ))
    }
}
