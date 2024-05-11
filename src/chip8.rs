use std::fmt::Error;

use crate::memory::Ram;
pub struct Chip8{
    memory:Ram,
}

impl Chip8 {
    pub fn new() -> Chip8{
        Chip8 { memory: (Ram::new()) }
    }

    pub fn load_rom(&mut self,buf:Vec<u8>) ->Result<(),String> {
        let offset = 0x200;
        for i in 0..buf.len() {
            self.memory.write_byte((offset+i) as u16,buf[i])?;
        }
        Ok(())

    }

}