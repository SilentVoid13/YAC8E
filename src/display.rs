
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    screen: Vec<Vec<u8>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: vec![vec![0; WIDTH]; HEIGHT],
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

    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.screen[y][x] = 0;
            }
        }
    }
}