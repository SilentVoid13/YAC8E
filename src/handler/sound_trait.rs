use std::fmt::Debug;

/// Sound functions that a handler must implement
pub trait SoundTrait: Debug {
    /// Plays a beep sound
    fn play_beep(&self);
    /// Stops the beep sound
    fn stop_beep(&self);
}