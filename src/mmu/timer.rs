pub struct Timer {
    pub timer_irq: bool,
    pub enabled: bool,

    cycles_since_div: u8,
    cycles_since_tima: u32,
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            enabled: true,

            timer_irq: false,
            div: 0,
            cycles_since_div: 0,
            cycles_since_tima: 0,
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

        if self.enabled {
            self.cycles_since_tima += n as u32;

            while self.cycles_since_tima >= 256 {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.timer_irq = true
                }
                self.cycles_since_tima -= 256;
            }
        }
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
