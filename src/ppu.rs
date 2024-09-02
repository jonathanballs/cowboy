pub struct PPU {
    ticks: u32,
}

impl PPU {
    pub fn do_cycle(&mut self, ticks: u32) {
        self.ticks += ticks;
    }
}
