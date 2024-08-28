#[derive(Debug)]
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
