
extern crate minifb;
use cpu::CPU;
mod cpu;
mod video;
//mod keypad;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
    let mut buffer = [0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "CHIP-8 emulator - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cpu = CPU::new();
    cpu.load_rom("test_opcode.ch8");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //execute cpu cycles
        cpu.emulate_cycle();
        // have a way to scale cpu frame buffer to window frame buffer


        // update window by reading frame buffer
        // for i in buffer.iter_mut() {
        //     if window.is_key_down(Key::Up) {
        //         *i = 0xffffff;
        //     } else {
        //         *i = 0x000000;
        //     }


            // println!("{:?}",pressed_keys(&window));
             // write something more funny here!
        
        //println!("{:?}",pressed_keys(&window));
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


fn pressed_keys(window: &Window) -> [bool;16] {
    let mut keys_pressed = [false;16];
    //let window = Window::new();
    window.get_keys().map(|keys| {
        for key in keys {
            match key {
                Key::Key1 => {keys_pressed[0x1] = true},
                Key::Key2 => {keys_pressed[0x2] = true},
                Key::Key3 => {keys_pressed[0x3] = true},
                Key::Key4 => {keys_pressed[0xC] = true},
                Key::Q => {keys_pressed[0x4] = true},
                Key::W => {keys_pressed[0x5] = true},
                Key::E => {keys_pressed[0x6] = true},
                Key::R => {keys_pressed[0xD] = true},
                Key::A => {keys_pressed[0x7] = true},
                Key::S => {keys_pressed[0x8] = true},
                Key::D => {keys_pressed[0x9] = true},
                Key::F => {keys_pressed[0xE] = true},
                Key::Z => {keys_pressed[0xA] = true},
                Key::X => {keys_pressed[0x0] = true},
                Key::C => {keys_pressed[0xB] = true},
                Key::V => {keys_pressed[0xF] = true},
                _ => ()
                }
            }
        });
    keys_pressed
    
}

