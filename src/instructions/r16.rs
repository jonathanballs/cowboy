#[derive(Debug)]
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
