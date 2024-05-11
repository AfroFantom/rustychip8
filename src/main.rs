use std::{env, fs::File};
use std::io::prelude::*;
extern crate sdl2;

mod memory;
mod chip8;

fn main() -> Result<(),String> {
    let args: Vec<_>=env::args().collect();
    let pathstring: String = "data/".to_owned()+&args[1];
    let mut rom_file = File::open(pathstring.as_str())?;
    let mut buf:Vec<u8> = Vec::new();
    let bytesize = rom_file.read_to_end(&mut buf)?;
    println!("bytes: {}\n content:{:?}\n",bytesize,buf);
    let mut chip8:chip8::Chip8 = chip8::Chip8::new();
    chip8.load_rom(buf)?;
    Ok(())
}
