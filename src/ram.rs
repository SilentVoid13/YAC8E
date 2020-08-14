use crate::utils::{oob_write_error, oob_read_error, integer_overflow_error};

use std::error::Error;

#[derive(Debug)]
/// Struct emulating the CHIP-8 RAM
pub struct Ram {
    /// Memory vector
    memory: Vec<u8>,
}

impl Ram {
    /// Creates a new `Ram` object
    pub fn new() -> Self {
        let mut ram = Ram {
            memory: vec![0u8; 4096],
        };
        // We load the builtins sprites
        Ram::load_sprites(&mut ram.memory);

        ram
    }

    /// Loads the builtins font utilities to allow for simple output of common characters in memory
    /// To use these sprites, the opcode FX29 must be used
    fn load_sprites(memory: &mut [u8]) {
        let sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        memory[0..sprites.len()].copy_from_slice(&sprites);
    }

    /// Writes a single byte into memory at `address` with value `value`
    pub fn write_byte(&mut self, address: usize, value: u8) -> Result<(), Box<dyn Error>> {
        *self.memory.get_mut(address).ok_or(
            oob_write_error(address, &[value])
        )? = value;
        Ok(())
    }

    #[allow(dead_code)]
    /// Writes multiple bytes into memory at `address` with value `buf`
    pub fn write_bytes(&mut self, address: usize, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.memory
            .get_mut(
                address..address.checked_add(buf.len()).ok_or(
                    integer_overflow_error(address, buf.len())
                )?
            )
            .ok_or(
                oob_write_error(address, buf)
            )?
            .copy_from_slice(buf);

        Ok(())
    }

    /// Reads a single byte at `address`
    pub fn read_byte(&self, address: usize) -> Result<u8, Box<dyn Error>>{
        let v = self.memory.get(address).ok_or(
            oob_read_error(address, 1)
        )?;
        Ok(*v)
    }

    #[allow(dead_code)]
    /// Reads `size` bytes at `address`
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], Box<dyn Error>>{
        let v = self.memory.get(address..address+size).ok_or(
            oob_read_error(address, size)
        )?;
        Ok(v)
    }
}