
const WIDTH: usize = 64;
const HEIGHT: usize = 32;


pub struct Display {
    // may cause issues later with oure renderer, because it is a u8
    mut display_buffer: [u8; WIDTH * HEIGHT],
}

impl Display {

    pub fn new() -> Display {
        Display {
            display_buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
    }
    pub fn draw(&mut self, &index:(usize, usize), color:u8) {
        self.display_buffer[]
    }
}

