use std::error::Error;

/// Width of the screen
pub const WIDTH: usize = 64;
/// Height of the screen
pub const HEIGHT: usize = 32;

#[derive(Debug)]
/// Struct containing all the pixels of the screen
pub struct Screen {
    /// Vector containing vectors. Each of these vectors represent a row of the screen.
    pub pixels: Vec<Vec<u8>>,
}

impl Screen {
    /// Creates a new `Screen` object
    pub fn new() -> Self {
        Screen {
            pixels: vec![vec![0; WIDTH]; HEIGHT],
        }
    }

    /// Draws the sprite `byte` to the screen at coordinates (`x`,`y`)
    /// For a sprite data byte, a bit set to one corresponds to a colored pixel (=white).
    /// Contrastingly, a bit set to zero corresponds to a transparent pixel (=black)
    ///
    /// Returns whether the drawing erased an existing byte so the CPU can set VF accordingly (1 if erased, 0 if not)
    pub fn draw_byte(&mut self, x: u8, y: u8, mut byte: u8) -> Result<bool, Box<dyn Error>> {
        let mut erased = false;

        let x= (x % WIDTH as u8) as usize;
        let y = (y % HEIGHT as u8) as usize;

        for i in 0..8 {
            let coord_x = (x+i) % WIDTH;
            let prev_value = self.pixels[y][coord_x];
            let bit = byte >> 7;

            self.pixels[y][coord_x] ^= bit;

            if prev_value == 1 && self.pixels[y][coord_x] == 0 {
                erased = true;
            }

            byte <<= 1;
        }

        Ok(erased)
    }

    /// Clears the screen, resetting all pixels to 0
    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for p in row.iter_mut() {
                *p = 0;
            }
        }
    }

    #[allow(dead_code)]
    /// Displays the screen on the terminal, mainly for debug purpose
    pub fn terminal_display(&self) {
        for row in self.pixels.iter() {
            for &col in row.iter() {
                if col == 1 {
                    print!("# ");
                }
                else {
                    print!("- ");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}