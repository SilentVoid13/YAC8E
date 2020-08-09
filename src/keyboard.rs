
#[derive(Debug)]
pub struct Keyboard {
    pub key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            key_pressed: None,
        }
    }

    pub fn is_key_pressed(&self, _key_code: u8) -> bool {
        /*
        if let Some(k) = self.key_pressed.as_ref() {
            k == &key_code
        }
        else {
            false
        }
         */
        false
    }
}