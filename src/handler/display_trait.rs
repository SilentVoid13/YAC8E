use std::error::Error;
use std::fmt::Debug;

/// Display functions that a handler must implement
pub trait DisplayTrait: Debug {
    /// Updates the screen to display new changes
    fn update(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>;
    /// Draws the `pixels` to the active window / canvas. Does not refresh the screen however
    fn draw(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>;
}