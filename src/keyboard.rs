
#[derive(Debug)]
/// The keyboard component, handling the keystrokes
pub struct Keyboard {
    pub key_pressed: Option<u8>,
}

impl Keyboard {
    /// Creates a new `Keyboard` object
    pub fn new() -> Self {
        Keyboard {
            key_pressed: None,
        }
    }

    /// Returns `true` if `key_code` corresponds to `key_pressed`, `false` otherwise
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