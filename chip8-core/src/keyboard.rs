extern crate piston_window;
use piston_window::*;

pub struct Keyboard {
    keys: [bool; 16],
}

// Mapping of Chip8's native keys to host keys
fn map_key(key: Key) -> u8 {
    match key {
        Key::D1 => 0x01,
        Key::D2 => 0x02,
        Key::D3 => 0x03,
        Key::D4 => 0x0C,
        Key::Q  => 0x04,
        Key::W  => 0x05,
        Key::E  => 0x06,
        Key::R  => 0x0D,
        Key::A  => 0x07,
        Key::S  => 0x08,
        Key::D  => 0x09,
        Key::F  => 0x0E,
        Key::Z  => 0x0A,
        Key::X  => 0x00,
        Key::C  => 0x0B,
        Key::V  => 0x0F,
        _ => 0x00, // Map other keys to 0 for now...
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: [false; 16] }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        self.keys[map_key(key) as usize] = true;
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keys[map_key(key) as usize] = false;
    }
}


