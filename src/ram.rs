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

    /// Loads the buit-ins font utilities to allow for simple output of common characters in memory
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

    pub fn write_byte(&mut self, address: usize, value: u8) {
        // TODO: Secure this
        self.memory[address] = value;
    }

    #[allow(dead_code)]
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
        //let v = self.memory.get(address).ok_or("OOB index")?;
        //Ok(*v)

        // TODO: Secure this
        Ok(self.memory[address])
    }

    #[allow(dead_code)]
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], Box<dyn Error>>{
        let v = self.memory.get(address..address+size).ok_or("OOB index")?;
        Ok(v)
    }
}