use crate::handler::display;
use crate::handler::display::{WIDTH, HEIGHT, DisplayTrait};

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Window};

#[derive(Debug)]
/// The display component, handling the 64x32 pixels screen
pub struct MiniFbDisplay {
    pub window: Rc<RefCell<Window>>,
    /// Vector containing all of the pixels of the screen
    pub screen: Vec<u8>,
}

impl MiniFbDisplay {
    /// Creates a new `Display` object
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        MiniFbDisplay {
            window: window,
            screen: vec![0; WIDTH * HEIGHT],
        }
    }
}

impl DisplayTrait for MiniFbDisplay {
    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.window.borrow_mut().update_with_buffer(&display::color_screen(&self.screen), WIDTH, HEIGHT)?;
        Ok(())
    }

    /// Draws the sprite `byte` to the screen at coordinates (`x`,`y`)
    /// For a sprite data byte, a bit set to one corresponds to a white pixel. Contrastingly, a bit set to zero corresponds to a transparent pixel
    ///
    /// Returns whether the drawing erased an existing byte so the CPU can set VF accordingly (1 if erased, 0 if not)
    fn draw_byte(&mut self, x: u8, y: u8, byte: u8) -> Result<bool, Box<dyn Error>> {
        display::draw_byte(&mut self.screen, x, y, byte)
    }

    /// Clears the screen, resetting all pixels to 0
    fn clear(&mut self) {
        display::clear(&mut self.screen);
    }
}
