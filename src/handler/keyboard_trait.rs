use std::fmt::Debug;

/// Keyboard functions that a handler must implement
pub trait KeyboardTrait: Debug {
    /// Update `keys_state`, `true` if key is pressed, `false` if key is released
    fn update_keys_state(&mut self, keys_state: &mut [bool]) -> bool;
}



