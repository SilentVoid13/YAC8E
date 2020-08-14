use std::fmt::Debug;

pub trait SoundTrait: Debug {
    fn play_beep(&self);
    fn stop_beep(&self);
}