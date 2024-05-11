use std::{fmt::Error, io::ErrorKind};

pub struct Ram{
    mem:[u8;4096],
}

impl Ram{
    pub fn new() -> Ram{
        Ram { mem: ([0;4096]) }
    }
    //write to a 16bit addr with a u8 value retur
    pub fn write_byte(&mut self,addr:u16,value:u8) -> Result<(),String> {
        if addr <4096 {
            self.mem[addr as usize] = value;
            Ok(())
        }else{
            Err("Out of bounds mem check".to_string())
        }
    }
    //read in a 16 bit addr and return a u8 data byte
    //pub fn read_byte(addr:u16) ->Option<u8> {}

}