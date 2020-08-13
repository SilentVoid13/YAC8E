use std::error::Error;
use std::fmt::Debug;

pub trait DisplayTrait: Debug {
    fn update(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>;
    fn draw(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>;
}