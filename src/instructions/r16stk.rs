#[derive(Debug, Clone, Copy)]
pub enum R16stk {
    BC,
    DE,
    HL,
    AF,
}

impl std::convert::From<u8> for R16stk {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HL,
            _ => Self::AF,
        }
    }
}
