pub struct PPU {
    scy: u8,
    scx: u8,
    line: u8,

    lcdc: u8,

    // Pallettes
    bgp: u8,

    modeclock: u32,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            scy: 0,
            scx: 0,
            bgp: 0,
            line: 0,
            lcdc: 0,
            modeclock: 32,
        }
    }

    //pub fn do_cycle(&mut self, ticks: u32) {
    //    self.ticks += ticks;
    //}

    pub fn do_cycle(&mut self, ticks: u32) {
        let mut ticksleft = ticks;

        while ticksleft > 0 {
            let curticks = if ticksleft >= 80 { 80 } else { ticksleft };
            self.modeclock += curticks;
            ticksleft -= curticks;

            // Full line takes 114 ticks
            if self.modeclock >= 456 {
                self.modeclock -= 456;
                self.line = (self.line + 1) % 154;
            }
        }
    }
    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcdc,
            0xFF42 => dbg!(self.scy),
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF47 => self.bgp,
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF40 => self.lcdc = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF47 => self.bgp = value,
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }
}
