use crate::cpu::{Cpu, PROGRAM_START};
use crate::display::{WIDTH, HEIGHT};
use crate::bus::Bus;
use crate::utils::log_debug;

use std::error::Error;
use std::fs::File;
use std::time::Duration;

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
            "Yet Another Chip-8 Emulator",
            window_width,
            window_height,
            WindowOptions::default(),
        )?;

        //let mut last_instruction_run_time = Instant::now();
        //let mut last_display_time = Instant::now();

        // Sets the refresh rate
        // minifb will check how much time has passed since the last time
        // and if it's less than the selected time it will sleep for the remainder of it.
        window.limit_update_rate(Some(Duration::from_micros(1000)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // We execute an instruction every 2 ms
            //if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            //chip8.run_instruction()?;
            //last_instruction_run_time = Instant::now();
            //}

            // Here we execute one instruction, then we update the window display
            // I don't know if it's better to separate these 2 steps into 2 separate timelines
            // e.g: Execute instruction every 2ms and try to refresh every 10 ms
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
        //print!(".");
        //io::stdout().flush();

        self.cpu.run_instruction(&mut self.bus, self.config.debug)?;

        if self.config.debug {
            log_debug(
                format!(
                    "Cpu state: {:#?}", self.cpu
                )
            );
        }

        Ok(())
    }
}