use crate::handler::{KEYBOARD_SIZE, keypad};
use crate::handler::keypad::KeypadTrait;

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Window, Key, KeyRepeat};

#[derive(Debug)]
/// The keyboard component, handling the keystrokes
pub struct MiniFbKeypad {
    pub window: Rc<RefCell<Window>>,
    pub keys_state: [bool; KEYBOARD_SIZE],
}

impl MiniFbKeypad {
    /// Creates a new `Keypad` object
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        MiniFbKeypad {
            window,
            keys_state: [false; KEYBOARD_SIZE],
        }
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

            _ => 0xFF,
        }
    }
}

impl KeypadTrait for MiniFbKeypad {
    /// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
    fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>> {
        keypad::is_key_pressed(&self.keys_state, key_code)
    }

    fn first_pressed_key(&self) -> Option<u8> {
        keypad::first_pressed_key(&self.keys_state)
    }

    fn update_keys_state(&mut self) {
        // Otherwise the closure grabs the whole self variable == double ownership on window
        let keys_state = &mut self.keys_state;
        self.window.borrow().get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                let k = MiniFbKeypad::convert_keycode(t);
                if k != 0xFF {
                    keys_state[k as usize] = true;
                }
            }
        });
        self.window.borrow().get_keys_released().map(|keys| {
            for t in keys {
                let k = MiniFbKeypad::convert_keycode(t);
                if k != 0 {
                    keys_state[k as usize] = false;
                }
            }
        });
    }

    /// Checks whether the display / user is sending an exit msg
    fn must_quit(&mut self) -> bool {
        !self.window.borrow().is_open() || self.window.borrow().is_key_down(Key::Escape)
    }
}
