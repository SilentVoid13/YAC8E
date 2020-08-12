use crate::handler::display;

use std::error::Error;
use std::fmt::Debug;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub trait DisplayTrait: Debug {
    fn update(&mut self) -> Result<(), Box<dyn Error>>;
    fn clear(&mut self);
    fn draw_byte(&mut self, x: u8, y: u8, byte: u8) -> Result<bool, Box<dyn Error>>;
}

/// Draws the sprite `byte` to the screen at coordinates (`x`,`y`)
/// For a sprite data byte, a bit set to one corresponds to a white pixel. Contrastingly, a bit set to zero corresponds to a transparent pixel
///
/// Returns whether the drawing erased an existing byte so the CPU can set VF accordingly (1 if erased, 0 if not)
pub fn draw_byte(screen: &mut [u8], mut x: u8, mut y: u8, mut byte: u8) -> Result<bool, Box<dyn Error>> {
    let mut erased = false;

    x %= WIDTH as u8;
    y %= HEIGHT as u8;

    for mut coord_x in x..x+8 {
        coord_x %= WIDTH as u8;

        let index = display::get_index_from_coords(coord_x as usize, y as usize);
        let prev_value = screen[index];

        let bit = byte >> 7;
        screen[index] ^= bit;

        if prev_value == 1 && screen[index] == 0 {
            erased = true;
        }

        byte <<= 1;
    }

    Ok(erased)
}

/// Clears the screen, resetting all pixels to 0
pub fn clear(screen: &mut [u8]) {
    for p in screen.iter_mut() {
        *p = 0;
    }
}

/// Gets the index for the `screen` vector based on coordinates (`x`,`y`)
pub fn get_index_from_coords(x: usize, y: usize) -> usize {
    (y * WIDTH) + x
}

#[allow(dead_code)]
/// Displays the screen on the terminal, mainly for debug purpose
pub fn terminal_display(screen: &[u8]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = display::get_index_from_coords(x, y);
            if screen[index] == 1 {
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
pub fn color_screen(screen: &[u8]) -> Vec<u32> {
    screen.iter().map(|b| {
        if *b == 1 {
            0xffffff
        }
        else {
            0x0
        }
    }).collect::<Vec<u32>>()
}