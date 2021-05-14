use minifb::{Key, Window};


pub struct KeyPad {
    keys_pressed: [bool; 16]
}

impl KeyPad {
    pub fn new() -> KeyPad {
        KeyPad {
            keys_pressed: [false; 16]
        }
    }
    pub fn is_pressed(&mut self, index: usize) -> bool {
        self.keys_pressed[index]
    }

    fn pressed_keys(&mut self) -> [bool;16] {
        //let mut keys_pressed = [false;16];
        Window.get_keys().map(|keys| {
            for key in keys {
                match key {
                    Key::Key1 => {self.keys_pressed[0x1] = true},
                    Key::Key2 => {self.keys_pressed[0x2] = true},
                    Key::Key3 => {self.keys_pressed[0x3] = true},
                    Key::Key4 => {self.keys_pressed[0xC] = true},
                    Key::Q => {self.keys_pressed[0x4] = true},
                    Key::W => {self.keys_pressed[0x5] = true},
                    Key::E => {self.keys_pressed[0x6] = true},
                    Key::R => {self.keys_pressed[0xD] = true},
                    Key::A => {self.keys_pressed[0x7] = true},
                    Key::S => {self.keys_pressed[0x8] = true},
                    Key::D => {self.keys_pressed[0x9] = true},
                    Key::F => {self.keys_pressed[0xE] = true},
                    Key::Z => {self.keys_pressed[0xA] = true},
                    Key::X => {self.keys_pressed[0x0] = true},
                    Key::C => {self.keys_pressed[0xB] = true},
                    Key::V => {self.keys_pressed[0xF] = true},
                    _ => ()
                    }
                }
            });
        self.keys_pressed
        
    }
}