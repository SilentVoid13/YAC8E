use std::error::Error;
use std::fmt::Debug;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

/// The display component, handling the 64x32 pixels screen
pub struct Display {
    /// Vector containing all of the pixels of the screen
    pub screen: Vec<u32>,
}

pub trait DisplayTrait: Debug {
    fn update(&mut self) -> Result<(), Box<dyn Error>>;
    fn clear(&mut self);
    fn draw_byte(&mut self, x: u8, y: u8, byte: u8) -> Result<bool, Box<dyn Error>>;
}

impl Display {
    /// Creates a new `Display` object
    pub fn new() -> Self {
        Display {
            screen: vec![0; WIDTH * HEIGHT],
        }
    }

    /// Draws the sprite `byte` to the screen at coordinates (`x`,`y`)
    /// For a sprite data byte, a bit set to one corresponds to a white pixel. Contrastingly, a bit set to zero corresponds to a transparent pixel
    ///
    /// Returns whether the drawing erased an existing byte so the CPU can set VF accordingly (1 if erased, 0 if not)
    pub fn draw_byte(&mut self, mut x: u8, mut y: u8, mut byte: u8) -> Result<bool, Box<dyn Error>> {
        let mut erased = false;

        x %= WIDTH as u8;
        y %= HEIGHT as u8;

        for mut coord_x in x..x+8 {
            coord_x %= WIDTH as u8;

            let index = Display::get_index_from_coords(coord_x as usize, y as usize);
            let prev_value = self.screen[index];

            let bit = byte >> 7;
            self.screen[index] ^= bit as u32;

            if prev_value == 1 && self.screen[index] == 0 {
                erased = true;
            }

            byte <<= 1;
        }
        //self.terminal_display();

        Ok(erased)
    }

    /// Clears the screen, resetting all pixels to 0
    pub fn clear(&mut self) {
        for p in self.screen.iter_mut() {
            *p = 0;
        }
    }

    /// Gets the index for the `screen` vector based on coordinates (`x`,`y`)
    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    #[allow(dead_code)]
    /// Displays the screen on the terminal, mainly for debug purpose
    pub fn terminal_display(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Display::get_index_from_coords(x, y);
                if self.screen[index] == 1 {
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

    /// Converts the binary screen (0 or 1) to a color screen (black / white)
    pub fn color_screen(&self) -> Vec<u32> {
        self.screen.clone().iter_mut().map(|b| {
            if *b == 1 {
                0xffffff
            }
            else {
                0x0
            }
        }).collect::<Vec<u32>>()
    }
}