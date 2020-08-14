use crate::handler::display_trait::DisplayTrait;

use std::error::Error;
use std::time::{Instant, Duration};
use std::thread;

use core::fmt;

use sdl2::{Sdl};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect};
use sdl2::pixels::Color;

const SCALE_FACTOR: u32 = 10;

pub struct SdlDisplay {
    display_rate: Duration,
    prev_time: Instant,
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

    fn clear(&mut self) {
        self.canvas.clear();
    }
}

impl DisplayTrait for SdlDisplay {
    fn update(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        self.update_rate();
        self.draw(pixels)?;
        self.canvas.present();
        Ok(())
    }

    fn draw(&mut self, pixels: &Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        self.clear();

        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate()  {
                let coord_x = x as u32 * SCALE_FACTOR;
                let coord_y = y as u32 * SCALE_FACTOR;

                if col == 0 {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                else if col == 1 {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }

                self.canvas.fill_rect(Rect::new(coord_x as i32, coord_y as i32, SCALE_FACTOR, SCALE_FACTOR))?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for SdlDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlDisplay")
            .field("display_rate", &self.display_rate)
            .field("prev_time", &self.prev_time)
            .finish()
    }
}
