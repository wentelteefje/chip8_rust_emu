extern crate piston_window;
use piston_window::*;

pub struct Keyboard {
    keys: [bool; 16],
    waiting_for_key_release: bool,
    key_register: Option<usize>
}

// Mapping of Chip8's native keys to host keys
fn map_key(key: Key) -> Option<usize> {
    match key {
        Key::D1 => Some(0x01),
        Key::D2 => Some(0x02),
        Key::D3 => Some(0x03),
        Key::D4 => Some(0x0C),
        Key::Q => Some(0x04),
        Key::W => Some(0x05),
        Key::E => Some(0x06),
        Key::R => Some(0x0D),
        Key::A => Some(0x07),
        Key::S => Some(0x08),
        Key::D => Some(0x09),
        Key::F => Some(0x0E),
        Key::Z => Some(0x0A),
        Key::X => Some(0x00),
        Key::C => Some(0x0B),
        Key::V => Some(0x0F),
        _ => None, // Map other keys to 0 for now...
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: [false; 16], waiting_for_key_release: false, key_register: None }
    }

    pub fn get_keys(&mut self) -> [bool; 16] {
        self.keys
    }

    pub fn get_key_register(&self) -> Option<usize> {
        self.key_register
    }

    pub fn set_waiting_for_key_release(&mut self) {
        self.waiting_for_key_release = true;
    }

    pub fn key_is_pressed(&self, key: u8) -> bool {
        assert!(key <= 0x0F);
        self.keys[key as usize]
    }

    pub fn handle_key_press(&mut self, key: Key) {
        let mapped_key = map_key(key);
        if let Some(key) = mapped_key {
            self.keys[key as usize] = true;
        }
        
    }

    pub fn handle_key_release(&mut self, key: Key) {
        let mapped_key = map_key(key);
        if let Some(key) = mapped_key {
            self.keys[key] = false;
            if self.waiting_for_key_release {
                self.key_register = Some(key);
                self.waiting_for_key_release = false;
            }
        }
    }
}