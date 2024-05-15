use std::{fmt::Error, io::ErrorKind};

pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram { mem: [0; 4096] };
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], //0
            [0x20, 0x60, 0x20, 0x20, 0x70], //1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], //2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], //3
            [0x90, 0x90, 0xF0, 0x10, 0x10], //4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], //5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], //6
            [0xF0, 0x10, 0x20, 0x40, 0x40], //7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], //8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], //9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], //A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], //B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], //C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], //D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], //E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], //F
        ];
        let mut i: usize = 0;
        let font_offset: usize = 0x50;
        for sprite in sprites.iter() {
            for hex in sprite {
                ram.mem[i + font_offset] = *hex;
                i += 1;
            }
        }
        ram
    }
    //write to a 16bit addr with a u8 value retur
    pub fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
        if addr < 4096 {
            self.mem[addr as usize] = value;
            Ok(())
        } else {
            Err("Out of bounds mem (write) check".to_string())
        }
    }
    //read out a 16 bit addr and return a u8 data byte
    pub fn read_byte(&self, addr: u16) -> Result<u8, String> {
        if addr < 4096 {
            Ok(self.mem[addr as usize])
        } else {
            Err("Out of bounds mem (read) check".to_string())
        }
    }
}
