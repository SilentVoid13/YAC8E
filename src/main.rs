mod chip8;
mod ram;
mod cpu;
mod bus;
mod keyboard;
mod display;
mod utils;

use crate::chip8::Chip8;
use crate::display::{WIDTH, HEIGHT};

use std::fs::File;
use std::io::Read;
use std::error::Error;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>>{
    let mut file = File::open("games/INVADERS")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data)?;

    let window_width = WIDTH * 10;
    let window_height = HEIGHT * 10;
    let mut window = Window::new(
        "Yet Another Chip-8 Emulator",
        window_width,
        window_height,
        WindowOptions::default(),
    )?;

    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    //window.limit_update_rate(Some(Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We execute an instruction every 2 ms
        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.run_instruction()?;
            last_instruction_run_time = Instant::now();
        }

        // We refresh display every 10 ms
        if Instant::now() - last_display_time > Duration::from_millis(10) {
            window.update_with_buffer(&chip8.bus.display.color_screen(), WIDTH, HEIGHT)?;
            last_display_time = Instant::now();
        }
    }

    Ok(())
}
