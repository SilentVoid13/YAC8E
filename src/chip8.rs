use crate::cpu::{Cpu, PROGRAM_START};

use std::error::Error;
use crate::bus::Bus;
use crate::utils::log_debug;

#[derive(Debug)]
pub struct Chip8 {
    pub bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.bus.ram.write_bytes(PROGRAM_START as usize, &data[..])?;

        Ok(())
    }

    pub fn run_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        self.cpu.run_instruction(&mut self.bus)?;

        log_debug(
            format!(
                "Cpu state: {:#?}", self.cpu
            )
        );

        Ok(())
    }
}