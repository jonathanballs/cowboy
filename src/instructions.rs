use std::fmt;

pub fn parse(opcode: u8, arg1: u8, arg2: u8) -> Instruction {
    if opcode == 0 {
        return Instruction::Nop;
    }

    // Block 3
    if opcode == 0xC3 {
        // little endian architecture so arg1 is smallest
        let imm16: u16 = (arg2 as u16) << 8 | arg1 as u16;
        return Instruction::Jmp(JmpType::JpImm16(imm16));
    }

    Instruction::INVALID
}

pub enum Instruction {
    Nop,
    Jmp(JmpType),
    INVALID,
}

//enum Condition {
//    Z,
//    NZ,
//    C,
//    NC,
//}

pub enum JmpType {
    JpHl,
    JpImm16(u16),
    //JpCondImm16(Condition, u16),
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "nop"),
            Self::INVALID => write!(f, "invalid"),
            Self::Jmp(JmpType::JpHl) => write!(f, "jphl"),
            Self::Jmp(JmpType::JpImm16(n)) => write!(f, "jp {:#06x}", &n),
        }
    }
}
