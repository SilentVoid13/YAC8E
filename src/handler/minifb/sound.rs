use crate::handler::sound_trait::SoundTrait;
use crate::utils::log_warning;

#[derive(Debug)]
/// Empty sound component, minifb doesn't handle sound
pub struct MiniFbSound {}

impl MiniFbSound {
    /// Creates a new `MiniFbSound` object
    pub fn new() -> Self {
        MiniFbSound{}
    }
}

impl SoundTrait for MiniFbSound {
    /// minifb doesn't handle sound
    fn play_beep(&self) {
        log_warning("MiniFb doesn't handle sound");
    }

    /// minifb doesn't handle sound
    fn stop_beep(&self) {}
}