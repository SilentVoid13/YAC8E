use crate::handler::sound_trait::SoundTrait;
use crate::utils::log_warning;

#[derive(Debug)]
pub struct MiniFbSound {}

impl MiniFbSound {
    pub fn new() -> Self {
        MiniFbSound{}
    }
}

impl SoundTrait for MiniFbSound {
    fn play_beep(&self) {
        log_warning("MiniFb doesn't handle sound");
    }

    fn stop_beep(&self) {}
}