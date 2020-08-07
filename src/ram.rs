use std::error::Error;

#[derive(Debug)]
pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Ram {
            memory: vec![0u8; 4096],
        };
        Ram::load_sprites(&mut ram.memory);

        ram
    }

    fn load_sprites(memory: &mut [u8]) {
        let mut sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let sprites = sprites.concat();
        memory[0..sprites.len()].copy_from_slice(&sprites);
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        // TODO: Secure this
        self.memory[address] = value;
    }

    pub fn write_bytes(&mut self, address: usize, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.memory
            .get_mut(
            address..address.checked_add(buf.len()).ok_or("Integer overflow")?
            )
            .ok_or("OOB index")?
            .copy_from_slice(buf);

        Ok(())
    }

    pub fn read_byte(&self, address: usize) -> Result<u8, Box<dyn Error>>{
        let v = self.memory.get(address).ok_or("OOB index")?;
        Ok(*v)
    }
}