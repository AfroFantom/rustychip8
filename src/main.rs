use std::io::prelude::*;
use std::{env, fs::File};
extern crate sdl2;
use display::Display;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

mod chip8;
mod cpu;
mod display;
mod memory;
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    let pathstring: String = "data/".to_owned() + &args[1];
    let mut rom_file;

    match File::open(pathstring.as_str()) {
        Ok(rom) => rom_file = rom,
        Err(_) => return Err("Could not open the specified path".to_string()),
    }

    let mut buf: Vec<u8> = Vec::new();
    let bytesize;
    match rom_file.read_to_end(&mut buf) {
        Ok(bytes) => bytesize = bytes,
        Err(_) => return Err("Could not read the rom file to the end".to_string()),
    }
    let mut chip8: chip8::Chip8 = chip8::Chip8::new();
    chip8.load_rom(&buf)?;
    chip8.print_rom(bytesize)?;
    //loaded example
    let mut display: Display = Display::new(WIDTH / 64 as usize, HEIGHT / 32 as usize);
    let mut buffer: Vec<u32> = display
        .clone()
        .getbuffer_as_u32();

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
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }
        //display.update()
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&display.clone().getbuffer_as_u32(), WIDTH, HEIGHT)
            .unwrap();
    }
    Ok(())
}
