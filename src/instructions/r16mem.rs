use std::fmt;

pub enum R16mem {
    BC,
    DE,
    HLI,
    HLD,
}

impl std::convert::From<u8> for R16mem {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HLI,
            _ => Self::HLD,
        }
    }
}

impl fmt::Debug for R16mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R16mem::BC => write!(f, "BC"),
            R16mem::DE => write!(f, "DE"),
            R16mem::HLI => write!(f, "[HL+]"),
            R16mem::HLD => write!(f, "[HL-]"),
        }
    }
}
