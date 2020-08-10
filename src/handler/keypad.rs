use std::error::Error;
use std::fmt::Debug;

pub const KEYBOARD_SIZE: usize = 16;

#[derive(Debug)]
/// The keyboard component, handling the keystrokes
pub struct Keypad {
    pub keys_state: [bool; KEYBOARD_SIZE],
}

pub trait KeypadTrait: Debug {
    fn update_keys_state(&mut self);
    fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>>;
    fn first_pressed_key(&self) -> Option<u8>;
}

impl Keypad {
    /// Creates a new `Keypad` object
    fn new<T: KeypadTrait>(handler: T) -> Self {
        Keypad {
            keys_state: [false; KEYBOARD_SIZE],
        }
    }

    /// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
    fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>> {
        self.keys_state.get(key_code as usize).ok_or("Invalid key code".into())
    }

    pub fn first_pressed_key(&self) -> Option<u8> {
        for (i, k) in self.keys_state.iter().enumerate() {
            if *k == true {
                return Some(i as u8);
            }
        }
        None
    }
}

