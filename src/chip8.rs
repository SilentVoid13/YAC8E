use crate::cpu::{Cpu, PROGRAM_START};
use crate::display::{WIDTH, HEIGHT};
use crate::bus::Bus;
use crate::utils::log_debug;

use std::error::Error;
use std::fs::File;
use std::time::{Duration, Instant};

use minifb::{Window, WindowOptions, Key};
use std::io::Read;

#[derive(Debug)]
/// The main struct containing all the components for the CHIP-8 VM
pub struct Chip8 {
    config: Chip8Config,
    pub bus: Bus,
    cpu: Cpu,
}

#[derive(Debug)]
/// A config struct containing various informations like the ROM path or if debug should be enabled
pub struct Chip8Config {
    pub rom: String,
    pub debug: bool,
    pub window_width: usize,
    pub window_height: usize,
}

impl Chip8 {
    /// Creates a new `Chip8` object given a `config`
    pub fn new(config: Chip8Config) -> Self {
        Chip8 {
            config: config,
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    /// Runs a ROM given a `chip8_config` parameter
    pub fn run_rom(chip8_config: Chip8Config) -> Result<(), Box<dyn Error>> {
        let mut chip8 = Chip8::new(chip8_config);

        let mut file = File::open(&chip8.config.rom)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        chip8.load_rom(&data)?;

        let window_width = chip8.config.window_width;
        let window_height = chip8.config.window_height;
        let mut window = Window::new(
            "Yet Another CHIP-8 Emulator",
            window_width,
            window_height,
            WindowOptions::default(),
        )?;

        // 500 Hz is considered a good value for CHIP-8 emulators.
        // This mean roughly that 1 clock cycle ~= 2ms
        // (This may vary depending on the instruction, e.g: drawing a sprite costs more than a simple XOR operation)
        let hz = 500.0;

        // Sets the refresh rate
        // minifb will check how much time has passed since the last time
        // and if it's less than the selected time it will sleep for the remainder of it.
        // minifb defaults to 4ms if not specified (quite slow)
        window.limit_update_rate(Some(Duration::from_secs_f32(1.0/hz)));

        let mut accumulator = Duration::new(0, 0);
        let mut last_time = Instant::now();
        let delta_cap = Duration::from_millis(3000);

        let frequency = Duration::from_secs_f32(1.0 / 60.0);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let current_time = Instant::now();
            let mut delta = current_time - last_time;
            // "Cap" the delta value in case the program gets stuck (e.g: waiting for a keystroke) so we don't have to simulate this long wait
            if delta > delta_cap {
                delta = delta_cap.clone();
            }
            last_time = current_time;
            accumulator += delta;

            while accumulator >= frequency {
                chip8.bus.update_timers(chip8.config.debug);
                accumulator -= frequency;
            }

            // We update the keys state (released / pressed)
            chip8.bus.keyboard.update_keys_state(&window);

            // Here we execute one instruction, then we update the window display
            // I don't know if it's better to separate these 2 steps into 2 separate timelines
            chip8.run_instruction()?;
            window.update_with_buffer(&chip8.bus.display.color_screen(), WIDTH, HEIGHT)?;
        }

        Ok(())
    }

    /// Loads the ROM data into RAM
    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.bus.ram.write_bytes(PROGRAM_START as usize, &data[..])?;

        Ok(())
    }

    /// Executes the instruction pointed by the PC
    pub fn run_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        self.cpu.run_instruction(&mut self.bus, self.config.debug)?;

        if self.config.debug {
            log_debug(
                format!(
                    "Cpu state: {:#?}", self.cpu
                )
            );
            log_debug(
                format!(
                    "Bus delay_timer: {:?}", self.bus.delay_timer
                )
            );
        }

        Ok(())
    }
}