use crate::handler::{KEYBOARD_SIZE, keypad};
use crate::handler::keypad::KeypadTrait;

use std::error::Error;

use core::fmt;

use sdl2::{Sdl, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct SdlKeypad {
    pub event_pump: EventPump,
    pub keys_state: [bool; KEYBOARD_SIZE],
}

impl SdlKeypad {
    pub fn new(sdl: &Sdl) -> Result<Self, Box<dyn Error>> {
        Ok(SdlKeypad {
            event_pump: sdl.event_pump()?,
            keys_state: [false; KEYBOARD_SIZE],
        })
    }

    /// Converts a SDL keycode to a CHIP-8 keycode
    fn convert_keycode(key: Keycode) -> u8 {
        match key {
            Keycode::Num1 => 0x1,
            Keycode::Num2 => 0x2,
            Keycode::Num3 => 0x3,
            Keycode::Num4 => 0xC,

            Keycode::Q => 0x4,
            Keycode::W => 0x5,
            Keycode::E => 0x6,
            Keycode::R => 0xD,

            Keycode::A => 0x7,
            Keycode::S => 0x8,
            Keycode::D => 0x9,
            Keycode::F => 0xE,

            Keycode::Z => 0xA,
            Keycode::X => 0x0,
            Keycode::V => 0xB,
            Keycode::C => 0xF,

            _ => 0xFF,
        }
    }
}

impl KeypadTrait for SdlKeypad {
    fn update_keys_state(&mut self) -> bool {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        println!("events: {:?}", events);
        for event in events {
            match event {
                Event::Quit {..} => {
                    return true;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(k) => {
                            if let Keycode::Escape = k {
                                return false;
                            }
                            let k = Self::convert_keycode(k);
                            if k != 0xFF {
                                self.keys_state[k as usize] = true;
                            }
                        }
                        _ => {},
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(k) => {
                            let k = Self::convert_keycode(k);
                            if k != 0xFF {
                                self.keys_state[k as usize] = false;
                            }
                        }
                        _ => {},
                    }
                },
                _ => {}
            }
        }

        true
    }

    fn is_key_pressed(&self, key_code: u8) -> Result<&bool, Box<dyn Error>> {
        keypad::is_key_pressed(&self.keys_state, key_code)
    }

    fn first_pressed_key(&self) -> Option<u8> {
        keypad::first_pressed_key(&self.keys_state)
    }
}

impl fmt::Debug for SdlKeypad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlKeypad")
            .field("keys_state", &self.keys_state)
            .finish()
    }
}
