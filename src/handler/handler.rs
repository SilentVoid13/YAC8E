use crate::ram::Ram;
use crate::handler::display::{Display, DisplayTrait};
use crate::utils::log_debug;
use crate::chip8::Chip8Config;
use crate::handler::keypad::KeypadTrait;
use crate::handler::minifb::{MiniFbKeypad, MiniFbDisplay};

use std::time::{Instant, Duration};
use minifb::{WindowOptions, Window};
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Handler {
    /// Keyboard component
    pub keypad: Box<dyn KeypadTrait>,
    /// Display component
    pub display: Box<dyn DisplayTrait>,
}

#[derive(Clone, Debug)]
pub enum HandlerType {
    MINIFB,
    SDL,
}

impl Handler {
    pub fn new(chip8_config: Chip8Config) -> Result<Self, Box<dyn Error>> {
        match chip8_config.handler_type {
            HandlerType::MINIFB => {
                let mut window = Window::new(
                    "Yet Another CHIP-8 Emulator",
                    chip8_config.window_width,
                    chip8_config.window_height,
                    WindowOptions::default(),
                )?;

                // Sets the refresh rate
                // minifb will check how much time has passed since the last time
                // and if it's less than the selected time it will sleep for the remainder of it.
                // minifb defaults to 4ms if not specified (quite slow)
                window.limit_update_rate(Some(Duration::from_secs_f64(1.0 / chip8_config.hertz)));

                let window = RefCell::new(window);

                Ok(Handler {
                    keypad: Box::new(MiniFbKeypad::new(RefCell::clone(&window))),
                    display: Box::new(MiniFbDisplay::new(RefCell::clone(&window))),
                })
            },
            HandlerType::SDL => {
                /*
                Handler {
                    keypad: Box::new(sdl::KeyPad::new(&chip8_config)),
                    display: Box::new(sdl::Display::new(&chip8_config)),
                }
                 */
                Err("SDL not implemented".into())
            },
        }
    }
}