use std::sync::atomic::{AtomicBool, Ordering};

use crate::gameboy::GameBoy;
pub static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

pub fn enable_debug() {
    DEBUG_MODE.store(true, Ordering::SeqCst);
}

pub fn disable_debug() {
    DEBUG_MODE.store(false, Ordering::SeqCst);
}

pub fn is_debug_enabled() -> bool {
    DEBUG_MODE.load(Ordering::SeqCst)
}

pub fn debugger_cli(gameboy: &mut GameBoy) {
    gameboy.debugger_cli();
}
