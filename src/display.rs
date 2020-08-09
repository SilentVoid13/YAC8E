use std::error::Error;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    pub screen: Vec<u32>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: vec![0; WIDTH * HEIGHT],
        }
    }

    /// Draws the byte to the screen
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

    pub fn clear(&mut self) {
        for p in self.screen.iter_mut() {
            *p = 0;
        }
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    #[allow(dead_code)]
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