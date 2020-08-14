use std::fmt::Debug;

pub trait KeyboardTrait: Debug {
    fn update_keys_state(&mut self, keys_state: &mut [bool]) -> bool;
}



