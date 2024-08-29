#[derive(Debug, Clone)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl std::convert::From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::HL,
            _ => Self::A,
        }
    }
}
