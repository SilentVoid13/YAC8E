use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;
use std::time::Instant;

#[derive(Debug)]
pub struct Bus {
    pub ram: Ram,
    pub keyboard: Keyboard,
    pub display: Display,
    delay_timer: u8,
    delay_timer_set_time: Instant,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
            delay_timer: 0,
            delay_timer_set_time: Instant::now(),
        }
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_set_time = Instant::now();
        self.delay_timer = value;
    }

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