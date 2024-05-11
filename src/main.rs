use std::{env, fs::File};
use std::io::prelude::*;
extern crate sdl2;

mod memory;
mod chip8;

fn main() -> Result<(),String> {
    let args: Vec<_>=env::args().collect();
    let pathstring: String = "data/".to_owned()+&args[1];
    let mut rom_file; 
    match File::open(pathstring.as_str()){
        Ok(rom) => rom_file=rom,
        Err(_) => {
            return Err("Could not open the specified path".to_string())
        },
    }
    let mut buf:Vec<u8> = Vec::new();
    let bytesize;
    match rom_file.read_to_end(&mut buf){
        Ok(bytes)=>bytesize=bytes,
        Err(_)=>{
            return Err("Could not read the rom file to the end".to_string())
        },
    }
    println!("bytes: {}\n content:{:?}\n",bytesize,buf);
    let mut chip8:chip8::Chip8 = chip8::Chip8::new();
    chip8.load_rom(buf)?;
    Ok(())
}
