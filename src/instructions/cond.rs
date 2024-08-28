#[derive(Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

impl std::convert::From<u8> for Cond {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => Self::NZ,
            1 => Self::Z,
            2 => Self::NC,
            _ => Self::C,
        }
    }
}
