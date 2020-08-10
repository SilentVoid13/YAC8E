use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;
use std::time::Instant;

#[derive(Debug)]
/// Bus struct interacting with system components
pub struct Bus {
    /// RAM of the VM
    pub ram: Ram,
    /// Keyboard component
    pub keyboard: Keyboard,
    /// Display component
    pub display: Display,
    /// Current delay timer
    delay_timer: u8,
    // TODO: Doc of this
    delay_timer_set_time: Instant,
}

impl Bus {
    /// Creates a new `Bus` object
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
            delay_timer: 0,
            delay_timer_set_time: Instant::now(),
        }
    }

    /// Sets a new delay timer
    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_set_time = Instant::now();
        self.delay_timer = value;
    }

    /// Get the current delay timer
    pub fn get_delay_timer(&self) -> u8 {
        let diff = Instant::now() - self.delay_timer_set_time;
        let ms = diff.as_millis();
        let ticks = ms / 16;
        if ticks >= self.delay_timer as u128 {
            0
        }
        else {
            self.delay_timer - ticks as u8
        }
    }
}