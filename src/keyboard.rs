use std::error::Error;

use minifb::{Window, Key, KeyRepeat};

const KEYBOARD_SIZE: usize = 16;

#[derive(Debug)]
/// The keyboard component, handling the keystrokes
pub struct Keyboard {
    pub keys_state: [bool; KEYBOARD_SIZE],
}

impl Keyboard {
    /// Creates a new `Keyboard` object
    pub fn new() -> Self {
        Keyboard {
            keys_state: [false; KEYBOARD_SIZE],
        }
    }

    /// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
    pub fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>> {
        self.keys_state.get(key_code as usize).ok_or("Invalid key code".into())
    }

    /// Converts a minifb keycode to a CHIP-8 keycode
    fn convert_keycode(key: Key) -> u8 {
        match key {
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::Key4 => 0xC,

            Key::Q => 0x4,
            Key::W => 0x5,
            Key::E => 0x6,
            Key::R => 0xD,

            Key::A => 0x7,
            Key::S => 0x8,
            Key::D => 0x9,
            Key::F => 0xE,

            Key::Z => 0xA,
            Key::X => 0x0,
            Key::C => 0xB,
            Key::V => 0xF,
            _ => 0,
        }
    }

    pub fn update_keys_state(&mut self, window: &Window) {
        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                let k = Keyboard::convert_keycode(t);
                if k != 0 {
                    self.keys_state[k as usize] = true;
                }
            }
        });
        window.get_keys_released().map(|keys| {
            for t in keys {
                let k = Keyboard::convert_keycode(t);
                if k != 0 {
                    self.keys_state[k as usize] = false;
                }
            }
        });
    }
}