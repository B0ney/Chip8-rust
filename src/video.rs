
const WIDTH: usize = 64;
const HEIGHT: usize = 32;


pub struct Display {
    pub display_buffer: [u8; WIDTH * HEIGHT],
}

impl Display {

    pub fn new() -> Display {
        Display {
            display_buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.display_buffer.iter_mut() {
            *pixel = 0;
        }
    }
    // my least favourite part of this
    pub fn draw(&mut self, x:usize, y:usize, sprite: &[u8]) -> u8 {
        let height = sprite.len();
        let mut pixel: u8;
        let mut collision: u8 = 0;

        for yline in 0..height {
            pixel = sprite[yline];

            for xline in 0..8 {
                let j = (y + yline) % HEIGHT;
                let i = (x + xline) % WIDTH;

                if (pixel & (0x80 >> xline)) != 0 {
                    if self.display_buffer[ i + j ] == 1 {
                        collision = 1;
                    }
                    self.display_buffer[i + (j * WIDTH)] ^= 1;
                }
            }
        }
        collision
    }
}

