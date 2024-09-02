use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct PPUState {
    frame_number: u32,
    scy: u8,
    scx: u8,
    line: u8,
    lcdc: u8,
    bgp: u8,
    modeclock: u32,
}

pub struct PPU {
    tx: Sender<PPUState>,
    state: PPUState,
}

impl PPU {
    pub fn new(tx: Sender<PPUState>) -> PPU {
        PPU {
            tx,
            state: PPUState {
                scy: 0,
                scx: 0,
                bgp: 0,
                line: 0,
                lcdc: 0,
                modeclock: 32,
                frame_number: 1,
            },
        }
    }

    pub fn do_cycle(&mut self, ticks: u32) {
        let mut ticksleft = ticks;

        while ticksleft > 0 {
            let curticks = if ticksleft >= 80 { 80 } else { ticksleft };
            self.state.modeclock += curticks;
            ticksleft -= curticks;

            // Full line takes 114 ticks
            if self.state.modeclock >= 456 {
                self.state.modeclock -= 456;
                self.state.line = (self.state.line + 1) % 154;
                if self.state.line == 0 {
                    self.state.frame_number += 1;
                    println!("frame");
                    self.tx.send(self.state.clone()).unwrap();
                }
            }
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.state.lcdc,
            0xFF42 => self.state.scy,
            0xFF43 => self.state.scx,
            0xFF44 => self.state.line,
            0xFF47 => self.state.bgp,
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF40 => self.state.lcdc = value,
            0xFF42 => self.state.scy = value,
            0xFF43 => self.state.scx = value,
            0xFF47 => self.state.bgp = value,
            _ => {
                dbg!(addr);
                todo!()
            }
        }
    }
}

