
extern crate minifb;
use cpu::CPU;
mod cpu;
mod video;
mod keypad;
use minifb::{Key, Window, WindowOptions};


const SCALE: usize = 10;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut buffer = [0u32; (WIDTH * HEIGHT)];
    let mut window = Window::new(
        "CHIP-8 emulator - ESC to exit - ENTER to restart",
        WIDTH * SCALE, 
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cpu = CPU::new();

    cpu.load_rom("PONG"); // hard coded for now
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Enter){
            cpu.reset_chip8();
        }
        cpu.io.update_keys(pressed_keys(&window));
        cpu.emulate_cycle();
        cpu.dt_dec();

        // updates video buffer
        for i in 0..buffer.len() {
            buffer[i] = if cpu.display.display_buffer[i % 2048] == 1 {0xffffff} else {0x000000}
        };

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

    };

    
            // println!("{:?}",pressed_keys(&window));
             // write something more funny here!
        
        //println!("{:?}",pressed_keys(&window));
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

}


fn pressed_keys(window: &Window) -> [bool;16] {
    let mut keys_pressed = [false;16];

    window.get_keys().map(|keys| {
        for key in keys {
            /*
            1 2 3 4
            Q W E R
            A S D F
            Z X C V     
            */
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

