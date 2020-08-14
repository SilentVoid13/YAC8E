use crate::cpu::{Cpu, PROGRAM_START};
use crate::ram::Ram;
use crate::screen::Screen;
use crate::keypad::Keypad;
use crate::handler::{Handler, HandlerType};
use crate::utils::log_debug;

use std::error::Error;
use std::fs::File;
use std::time::{Duration, Instant};
use std::io::Read;

#[derive(Debug)]
/// The main struct containing all the components of the CHIP-8 VM
pub struct Chip8 {
    /// CHIP-8 config
    config: Chip8Config,
    /// Handler for display, keyboard and sound
    handler: Handler,
    /// CPU of the VM
    cpu: Cpu,
    /// RAM of the VM
    ram: Ram,
    /// Screen of the VM
    screen: Screen,
    /// Keypad of the VM
    keypad: Keypad,
}

#[derive(Clone, Debug)]
/// A config struct containing the CHIP-8 configuration
pub struct Chip8Config {
    /// ROM path
    pub rom: String,
    /// Debugging output
    pub debug: bool,
    /// Handler type for display, keyboard and sound
    pub handler_type: HandlerType,
    /// Hertz value for CPU cycles per sec speed
    pub hertz: f64,
    /// Display window width
    pub window_width: usize,
    /// Display window height
    pub window_height: usize,
}

impl Chip8 {
    /// Creates a new `Chip8` object given a `Chip8Config`
    pub fn new(config: Chip8Config) -> Result<Self, Box<dyn Error>> {
        Ok(Chip8 {
            config: config.clone(),
            handler: Handler::new(config)?,
            cpu: Cpu::new(),
            ram: Ram::new(),
            screen: Screen::new(),
            keypad: Keypad::new(),
        })
    }

    /// Runs a ROM given a `Chip8Config`
    pub fn run_rom(chip8_config: Chip8Config) -> Result<(), Box<dyn Error>> {
        let mut chip8 = Chip8::new(chip8_config)?;

        // Reads ROM data
        let mut file = File::open(&chip8.config.rom)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        chip8.load_rom(&data)?;

        let mut accumulator = Duration::new(0, 0);
        let mut prev_time = Instant::now();
        let delta_cap = Duration::from_millis(3000);

        let timer_frequency = Duration::from_secs_f64(1.0 / 60.0);

        loop {
            // https://www.reddit.com/r/EmuDev/comments/7v7flo/duncetier_chip8_question_how_do_i_set_the_timers/
            let current_time = Instant::now();
            let mut delta = current_time - prev_time;
            // "Cap" the delta value in case the program gets stuck (e.g: waiting for a keystroke) so we don't have to simulate this long wait
            if delta > delta_cap {
                delta = delta_cap.clone();
            }
            prev_time = current_time;
            accumulator += delta;

            // With this, timers run at 60 Hz while the rest of the program can run at any speed
            while accumulator >= timer_frequency {
                let active_sound = chip8.cpu.update_timers(chip8.config.debug);
                accumulator -= timer_frequency;

                if active_sound {
                    chip8.handler.sound.play_beep();
                }
                else {
                    chip8.handler.sound.stop_beep();
                }
            }

            // We update the keys state (released / pressed), returns false if we receive an exit signal (Esc or Quit)
            if !chip8.handler.keyboard.update_keys_state(&mut chip8.keypad.keys_state) {
                break;
            }

            // Here we execute one instruction, then we update the window display, then we sleep if required (happens in the display.update() func)
            // Maybe it's better to separate these 2 steps into 2 separate timelines (i.e: Running more CPU instructions, then updating display)
            chip8.run_instruction()?;
            // Updates the screen and sleeps if necessary
            chip8.handler.display.update(&chip8.screen.pixels)?;
        }

        Ok(())
    }

    /// Loads the ROM data into RAM
    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.ram.write_bytes(PROGRAM_START as usize, &data[..])?;

        Ok(())
    }

    /// Executes the instruction pointed by the PC
    pub fn run_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        self.cpu.run_instruction(&mut self.ram, &mut self.screen, &self.keypad, self.config.debug)?;

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