use std::error::Error;
use std::fmt::Debug;

pub const KEYBOARD_SIZE: usize = 16;

pub trait KeypadTrait: Debug {
    fn update_keys_state(&mut self) -> bool;
    fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>>;
    fn first_pressed_key(&self) -> Option<u8>;
}

/// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
pub fn is_key_pressed(keys_state: &[bool], key_code: u8) -> Result<&bool, Box<dyn Error>> {
    keys_state.get(key_code as usize).ok_or("Invalid key code".into())
}

pub fn first_pressed_key(keys_state: &[bool]) -> Option<u8> {
    for (i, k) in keys_state.iter().enumerate() {
        if *k == true {
            return Some(i as u8);
        }
    }
    None
}

