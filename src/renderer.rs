use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

fn most_recent_frame(rx: &Receiver<Vec<u32>>) -> Option<Vec<u32>> {
    let mut latest_frame = None;

    loop {
        match rx.try_recv() {
            Ok(frame) => {
                // Update the latest frame
                latest_frame = Some(frame);
            }
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                panic!("");
            }
        }
    }

    latest_frame
}

pub fn window_loop(rx: Receiver<Vec<u32>>, tx: Sender<(bool, Key)>, game_title: &String) {
    let mut window = Window::new(
        format!("Cowboy Emulator - {}", game_title).as_str(),
        SCREEN_WIDTH as usize,
        SCREEN_HEIGHT as usize,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some(frame_buffer) = most_recent_frame(&rx) {
            window
                .update_with_buffer(&frame_buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
                .unwrap();
        }

        // dispatch unreleased keys
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| tx.send((true, *key)).unwrap());

        // dispatch released keys
        window
            .get_keys_released()
            .iter()
            .for_each(|key| tx.send((false, *key)).unwrap());
    }
}
