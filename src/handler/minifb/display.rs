use crate::handler::display_trait::{DisplayTrait};

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Window};

#[derive(Debug)]
/// Display component for minifb
pub struct MiniFbDisplay {
    /// Window handling all the events (display + keyboard)
    pub window: Rc<RefCell<Window>>,
}

impl MiniFbDisplay {
    /// Creates a new `Display` object
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        MiniFbDisplay {
            window: window,
        }
    }

    /// Converts the binary screen (0 or 1) to a color screen (black / white)
    /// Also converts the Vec<Vec<u8>> to a Vec<u32> for the update_with_buffer minifb function
    pub fn color_screen(&self, pixels: &Vec<Vec<u8>>) -> Vec<u32> {
        let v = pixels.iter().map(|p| {
            p.iter().map(|b| {
                if *b == 1 {
                    0xffffff
                }
                else {
                    0x0
                }
            }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
        v.concat()
    }
}

impl DisplayTrait for MiniFbDisplay {
    fn update(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let colored_flat_pixels = self.color_screen(pixels);
        self.window.borrow_mut().update_with_buffer(
            &colored_flat_pixels,
            pixels.get(0).ok_or("Empty pixels vector")?.len(),
            pixels.len()
        )?;
        Ok(())
    }

    /// We don't need to draw anything with minifb, only the pixels are required
    fn draw(&mut self, _pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
