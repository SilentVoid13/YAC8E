use crate::handler::sound_trait::SoundTrait;

use std::error::Error;

use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use sdl2::{Sdl};

use core::fmt;

// https://docs.rs/sdl2/0.34.2/sdl2/audio/index.html
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

/// Sound component for SDL
pub struct SdlSound {
    /// Audio device, on which the sound will be played
    audio_device: AudioDevice<SquareWave>,
}

impl SdlSound {
    /// Creates a new `SdlSound` object
    pub fn new(sdl: &Sdl) -> Result<Self, Box<dyn Error>> {
        let audio_subsystem = sdl.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // Initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        })?;

        Ok(SdlSound {
            audio_device: device,
        })
    }
}

impl SoundTrait for SdlSound {
    fn play_beep(&self) {
        self.audio_device.resume();
    }

    fn stop_beep(&self) {
        self.audio_device.pause();
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

/// Mock Debug implementation for debugging purpose
impl fmt::Debug for SdlSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdlSound")
            .finish()
    }
}

