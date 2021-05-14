extern crate minifb;
mod cpu;
use minifb::{Key, Window}

pub struct CHIP8 {

}

impl CHIP8 {
    pub fn run() {
        let mut buffer = [0; WIDTH * HEIGHT];
        
        // launch window
        let mut window = Window::new(
            "CHIP-8 emulator - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });


        let mut cpu = cpu::CPU::new();


    
        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        let mut colour:u8;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // for i in buffer.iter_mut() {
            //     *i = match window.is_key_down() {
            //             Key::Up => 0xffffff,
            //             Key::Down => 0x000000,
            //             _ => 0x808080,
            //         }
                //*i = 0xff4477; // write something more funny here!
                //emulator runs here
                // fetch decode execute cylce

            }
    
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}

