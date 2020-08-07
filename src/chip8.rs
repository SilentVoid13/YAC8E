use crate::ram::Ram;
use crate::cpu::{Cpu, PROGRAM_START};

use std::error::Error;

#[derive(Debug)]
pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.ram.write_bytes(PROGRAM_START as usize, &data[..])?;

        Ok(())
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram);

        println!("Cpu state: {:#?}", self.cpu);
    }
}