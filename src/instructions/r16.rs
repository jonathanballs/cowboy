use core::fmt;

pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

impl std::convert::From<u8> for R16 {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HL,
            _ => Self::SP,
        }
    }
}

impl fmt::Debug for R16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R16::BC => write!(f, "BC"),
            R16::DE => write!(f, "DE"),
            R16::SP => write!(f, "SP"),
            R16::HL => write!(f, "[HL]"),
        }
    }
}
