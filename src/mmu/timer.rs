pub struct Timer {
    pub timer_irq: bool,
    pub enabled: bool,

    cycles_since_div: u8,
    cycles_since_tima: u32,
    div: u8,
    tima: u8,
    tma: u8,
    step: u32,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer_irq: false,
            div: 0,
            cycles_since_div: 0,
            cycles_since_tima: 0,
            tima: 0,
            tma: 0,

            // tca
            step: 256,
            enabled: false,
        }
    }

    pub fn do_cycles(&mut self, n: u8) {
        if 0xff - self.cycles_since_div >= n {
            self.div = self.div.wrapping_add(1);
        }

        self.cycles_since_div = self.cycles_since_div.wrapping_add(n);

        if self.enabled {
            self.cycles_since_tima += n as u32;

            while self.cycles_since_tima >= self.step {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.timer_irq = true
                }

                self.cycles_since_tima -= self.step;
            }
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => {
                0xF8 | (if self.enabled { 0x4 } else { 0 })
                    | (match self.step {
                        16 => 1,
                        64 => 2,
                        256 => 3,
                        _ => 0,
                    })
            }

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
            0xFF07 => {
                self.enabled = value & 0x4 != 0;
                self.step = match value & 0x3 {
                    1 => 16,
                    2 => 64,
                    3 => 256,
                    _ => 1024,
                };
            }

            _ => unreachable!(),
        }
    }
}
