use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;

#[derive(Debug)]
pub struct Bus {
    pub ram: Ram,
    pub keyboard: Keyboard,
    pub display: Display,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
        }
    }
}