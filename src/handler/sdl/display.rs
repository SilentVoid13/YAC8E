use crate::handler::display::DisplayTrait;
use crate::handler::{WIDTH, HEIGHT, display};

use std::error::Error;
use std::time::{Instant, Duration};
use std::thread;

use core::fmt;

use sdl2::{Sdl, pixels};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;

const SCALE_FACTOR: u32 = 10;

pub struct SdlDisplay {
    display_rate: Duration,
    prev_time: Instant,
    /// Vector containing all of the pixels of the screen
    pub screen: Vec<u8>,
    canvas: Canvas<Window>,
}

impl SdlDisplay {
    pub fn new(sdl: &Sdl, display_frequency: Duration) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window("Chip 8 Emulator", 640, 320)
            .position_centered()
            .opengl()
            .build()?;

        let mut canvas = window
            .into_canvas()
            .build()?;
        canvas.clear();

        canvas.present();

        Ok(SdlDisplay {
            display_rate: display_frequency,
            prev_time: Instant::now(),
            screen: vec![0; WIDTH * HEIGHT],
            canvas: canvas,
        })
    }

    fn update_rate(&mut self) {
        let delta = self.prev_time.elapsed();
        if delta < self.display_rate {
            let sleep_time = self.display_rate - delta;
            thread::sleep(sleep_time);
        }
        self.prev_time = Instant::now();
    }
}

impl DisplayTrait for SdlDisplay {
    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.update_rate();
        self.canvas.present();
        Ok(())
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn draw_byte(&mut self, mut x: u8, mut y: u8, mut byte: u8) -> Result<bool, Box<dyn Error>> {
        self.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut erased = false;

        x %= WIDTH as u8;
        y %= HEIGHT as u8;


        for mut coord_x in x..x+8 {
            coord_x %= WIDTH as u8;

            let index = display::get_index_from_coords(coord_x as usize, y as usize);
            let prev_value = self.screen[index];

            let bit = byte >> 7;
            self.screen[index] ^= bit;

            if prev_value == 1 && self.screen[index] == 0 {
                erased = true;
            }
            if self.screen[index] == 1 {
                let coord_x = coord_x as u32 * SCALE_FACTOR;
                let coord_y = y as u32 * SCALE_FACTOR;

                //println!("drawing at ({}, {})", coord_x, coord_y);
                self.canvas.fill_rect(Rect::new(coord_x as i32, coord_y as i32, SCALE_FACTOR, SCALE_FACTOR));
                //std::thread::sleep(Duration::from_millis(200));
            }

            byte <<= 1;
        }

        Ok(erased)
    }
}

impl fmt::Debug for SdlDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlDisplay")
            .field("screen", &self.screen)
            .finish()
    }
}
