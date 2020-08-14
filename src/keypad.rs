use std::error::Error;

/// The max size of the keyboard (16 for CHIP-8)
pub const KEYBOARD_SIZE: usize = 16;

#[derive(Debug)]
/// Struct containing the active key strokes
pub struct Keypad {
    /// Array of the keys state. `true` if pressed, `false` if released.
    pub keys_state: [bool; KEYBOARD_SIZE],
}

impl Keypad {
    /// Creates a new `Keypad` object
    pub fn new() -> Self {
        Keypad {
            keys_state: [false; KEYBOARD_SIZE],
        }
    }

    /// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
    pub fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>> {
        self.keys_state.get(key_code as usize).ok_or("Invalid key code".into())
    }

    /// Returns the first pressed key we encounter, `None` otherwise
    pub fn first_pressed_key(&self) -> Option<u8> {
        for (i, k) in self.keys_state.iter().enumerate() {
            if *k == true {
                return Some(i as u8);
            }
        }
        None
    }
}