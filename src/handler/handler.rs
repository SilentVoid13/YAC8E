use crate::chip8::Chip8Config;
use crate::handler::display_trait::{DisplayTrait};
use crate::handler::keyboard_trait::KeyboardTrait;
use crate::handler::sound_trait::SoundTrait;
use crate::handler::minifb::{MiniFbKeyboard, MiniFbDisplay, MiniFbSound};
use crate::handler::sdl::{SdlKeyboard, SdlDisplay, SdlSound};

use std::time::{Duration};
use minifb::{WindowOptions, Window};
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Handler {
    /// Keyboard component
    pub keyboard: Box<dyn KeyboardTrait>,
    /// Display component
    pub display: Box<dyn DisplayTrait>,
    /// Sound component
    pub sound: Box<dyn SoundTrait>,
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

                // We share a mutable reference to the window.
                // This is safe because the application is single-threaded and we only use the mutable aspect of the window in MiniFbDisplay
                let window = Rc::new(RefCell::new(window));

                Ok(Handler {
                    keyboard: Box::new(MiniFbKeyboard::new(Rc::clone(&window))),
                    display: Box::new(MiniFbDisplay::new(Rc::clone(&window))),
                    sound: Box::new(MiniFbSound::new()),
                })
            },
            HandlerType::SDL => {
                let sdl = sdl2::init()?;
                let display_rate = Duration::from_secs_f64(1.0 / chip8_config.hertz);

                Ok(Handler {
                    keyboard: Box::new(SdlKeyboard::new(&sdl)?),
                    display: Box::new(SdlDisplay::new(&sdl, display_rate)?),
                    sound: Box::new(SdlSound::new(&sdl)?),
                })
            },
        }
    }
}