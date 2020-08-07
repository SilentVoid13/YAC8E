
#[derive(Debug)]
pub struct Display {
    screen:
}

impl Display {
    pub fn new() -> Self {
        Display {

        }
    }

    pub fn draw_sprite(&self, vx: u8, vy: u8, height: u8) {
        println!(
            "[DEBUG] Drawing a sprite at ({}, {}), of width {} and height {}",
            vx,
            vy,
            8,
            height,
        );

        for y in 0..height {
            //let byte = bus.ram_read_byte
        }
    }
}