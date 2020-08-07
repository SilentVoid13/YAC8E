mod chip8;
mod ram;
mod cpu;

use crate::chip8::Chip8;

use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let mut file = File::open("games/INVADERS")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("{:?}", data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data)?;

    loop {
        chip8.run_instruction();
    }

    Ok(())
}
