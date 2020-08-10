use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;
use std::time::{Instant};
use crate::utils::log_debug;

#[derive(Debug)]
/// Bus struct interacting with system components
pub struct Bus {
    /// RAM of the VM
    pub ram: Ram,
    /// Keyboard component
    pub keyboard: Keyboard,
    /// Display component
    pub display: Display,
    /// Delay timer
    pub delay_timer: u8,
    /// Sound timer
    pub sound_timer: u8,
    debug_time: Instant,
    debug_count: u64,
}

impl Bus {
    /// Creates a new `Bus` object
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
            delay_timer: 0,
            sound_timer: 0,
            debug_time: Instant::now(),
            debug_count: 0,
        }
    }

    /// Updates the timers (decrease by one)
    pub fn update_timers(&mut self, debug: bool) {
        if debug {
            self.debug_count += 1;
            // update_timers has been called 60 times
            if self.debug_count == 60 {
                // This should be 1 second
                let time_taken = Instant::now() - self.debug_time;
                self.debug_time = Instant::now();
                self.debug_count = 0;
                log_debug(
                    format!(
                        "Time taken for 60 update_timers: {:?}", time_taken
                    )
                );
            }
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}