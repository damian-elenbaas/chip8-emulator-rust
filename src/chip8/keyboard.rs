
pub struct Keyboard {
    key_pressed: Option<u8>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_pressed: None
        }
    }

    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        if let Some(key) = self.key_pressed {
            key == key
        } else {
            false
        }
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    } 
}
