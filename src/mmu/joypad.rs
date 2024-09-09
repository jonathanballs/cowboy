use minifb::Key;

pub struct Joypad {
    joypad: u8,
    ssba: u8,
    dulr: u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            joypad: 0x0,
            dulr: 0xF,
            ssba: 0xF,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // Joy pad
            0xFF00 => {
                return match (self.joypad >> 4) & 0x3 {
                    // Return both
                    0x0 => 0xC0 | (self.dulr & self.ssba),

                    // Return select
                    0x1 => 0xD0 | self.ssba,

                    // Return dpad
                    0x2 => 0xE0 | self.dulr,

                    // Return neither
                    0x3 => 0xFF,

                    _ => unreachable!(),
                };
            }

            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            0xFF00 => self.joypad = byte,
            _ => unreachable!(),
        }
    }

    pub fn handle_key_down(&mut self, key: Key) {
        match key {
            Key::Right => self.dulr &= !0x1,
            Key::Left => self.dulr &= !0x2,
            Key::Up => self.dulr &= !0x4,
            Key::Down => self.dulr &= !0x8,

            Key::S => self.ssba &= !0x1,
            Key::A => self.ssba &= !0x2,
            Key::Space => self.ssba &= !0x4,
            Key::Enter => self.ssba &= !0x8,

            _ => (),
        }
    }

    pub fn handle_key_up(&mut self, key: Key) {
        match key {
            Key::Right => self.dulr |= 0x1,
            Key::Left => self.dulr |= 0x2,
            Key::Up => self.dulr |= 0x4,
            Key::Down => self.dulr |= 0x8,

            Key::S => self.ssba |= 0x1,
            Key::A => self.ssba |= 0x2,
            Key::Space => self.ssba |= 0x4,
            Key::Enter => self.ssba |= 0x8,

            _ => (),
        }
    }
}
