use crate::handler::keyboard_trait::KeyboardTrait;

use std::error::Error;

use core::fmt;

use sdl2::{Sdl, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// The keyboard component, handling the keystrokes
pub struct SdlKeyboard {
    /// Event pump handling all the keyboard events
    pub event_pump: EventPump,
}

impl SdlKeyboard {
    /// Creates a new `SdlKeyboard` object
    pub fn new(sdl: &Sdl) -> Result<Self, Box<dyn Error>> {
        Ok(SdlKeyboard {
            event_pump: sdl.event_pump()?,
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

impl KeyboardTrait for SdlKeyboard {
    fn update_keys_state(&mut self, keys_state: &mut [bool]) -> bool {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit {..} => {
                    return false;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(k) => {
                            if let Keycode::Escape = k {
                                return false;
                            }
                            let k = Self::convert_keycode(k);
                            if k != 0xFF {
                                keys_state[k as usize] = true;
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
                                keys_state[k as usize] = false;
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
}

/// Mock Debug implementation for debugging purpose
impl fmt::Debug for SdlKeyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlKeyboard")
            .finish()
    }
}
