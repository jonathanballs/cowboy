pub struct Timer {
    cycles_since_div: u8,
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            cycles_since_div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn do_cycles(&mut self, n: u8) {
        if 0xff - self.cycles_since_div >= n {
            self.div = self.div.wrapping_add(1);
        }

        self.cycles_since_div = self.cycles_since_div.wrapping_add(n);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // Div register
            0xFF04 => self.div = 0x0,

            // Interrupt registers
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => unreachable!(),
        }
    }
}
