
pub struct Keypad {
    pub keys_pressed: [bool; 16]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys_pressed: [false; 16]
        }
    }
    pub fn is_pressed(&mut self, index: usize) -> bool {
        self.keys_pressed[index]
    }
    pub fn update_keys(&mut self, keys:[bool; 16]) {
        self.keys_pressed = keys;
    }
    pub fn get_key(&self) -> Option<u8> {
        for (i, &key_state) in self.keys_pressed.iter().enumerate() {
            if key_state == true {
                return Some(i as u8);
            };
        }
        None
    }
}