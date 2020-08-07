
#[derive(Debug)]
pub struct Keyboard {
    pub key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {

        }
    }

    pub fn key_pressed(key_code: u8) -> bool {
        true
    }
}