
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer = [0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            if window.is_key_down(Key::Up) {
                *i = 0xffffff;
            } else {
                *i = 0x000000;
            }
            
             // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
// fn get_colour(window: Window) -> &'static str {
//     window.get_keys().map(|keys| {
//         for t in keys {
//             match t {
//                 Key::W => "holding w",
//                 Key::T => "holding t",
//                 _ => (),
//             }
//         }
//     });
// }

// fn test(window: Window) ->&'static str{
//     match window.get_keys().unwrap() {
//         Key::Up => "holding up",
//         Key::Down => "holding down",
//         _ => "None"
//     }

// }