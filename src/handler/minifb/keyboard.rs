use crate::handler::keyboard_trait::KeyboardTrait;

use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Window, Key, KeyRepeat};

#[derive(Debug)]
/// The keyboard component, handling the keystrokes
pub struct MiniFbKeyboard {
    pub window: Rc<RefCell<Window>>,
}

impl MiniFbKeyboard {
    /// Creates a new `Keypad` object
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        MiniFbKeyboard {
            window,
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

impl KeyboardTrait for MiniFbKeyboard {
    fn update_keys_state(&mut self, keys_state: &mut [bool]) -> bool {
        if !self.window.borrow().is_open() || self.window.borrow().is_key_down(Key::Escape) {
            return false;
        }

        self.window.borrow().get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                let k = MiniFbKeyboard::convert_keycode(t);
                if k != 0xFF {
                    keys_state[k as usize] = true;
                }
            }
        });
        self.window.borrow().get_keys_released().map(|keys| {
            for t in keys {
                let k = MiniFbKeyboard::convert_keycode(t);
                if k != 0xFF {
                    keys_state[k as usize] = false;
                }
            }
        });

        true
    }
}
