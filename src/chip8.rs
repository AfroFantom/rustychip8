use std::fmt::Error;

use crate::cpu::Cpu;
use crate::memory::Ram;

pub struct Chip8 {
    memory: Ram,
    processor: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: (Ram::new()),
            processor: (Cpu::new()),
        }
    }

    pub fn load_rom(&mut self, buf: &Vec<u8>) -> Result<(), String> {
        let offset = 0x200;
        for i in 0..buf.len() {
            self.memory.write_byte((offset + i) as u16, buf[i])?;
        }
        Ok(())
    }

    pub fn print_rom(&self, bytesize: usize) -> Result<(), String> {
        let offset = 0x200;
        let mut lb = 0;
        for i in 0..(offset + bytesize) {
            lb += 1;
            print!(" {:#02x} ", self.memory.read_byte(i as u16)?);
            if lb > 14 {
                println!("");
                lb = 0;
            }
        }
        Ok(())
    }
}
