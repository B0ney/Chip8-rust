
const WIDTH: usize = 64;
const HEIGHT: usize = 32;


pub struct Display {
    // may cause issues later with oure renderer, because it is a u8
    pub display_buffer: [u8; WIDTH * HEIGHT],
    pub draw_flag: bool,
}

impl Display {

    pub fn new() -> Display {
        Display {
            display_buffer: [0; WIDTH * HEIGHT],
            draw_flag: false,
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

        for yline in 0..height{
            pixel = sprite[yline];

            for xline in 0..8 {
                let j = (y + yline);
                let i = (x + xline);

                if (pixel & (0x80 >> xline)) != 0 {
                    if self.display_buffer[ i + j ] == 1 {
                        collision = 1;
                    }
                    self.display_buffer[i + (j * 64)] ^= 1;
                }
            }
        }
        self.draw_flag = true;
        collision
    }
    // pub fn draw_to_buffer(scale:usize) {
    //     let size = (WIDTH * HEIGHT) * scale;
    //     let mut buffer = [u32: size];


    // }
}

