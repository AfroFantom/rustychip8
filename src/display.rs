const WIDTH: usize = 64;
const HEIGHT: usize = 32;
#[derive(Debug, Clone)]
pub struct Display {
    xmultiplier: usize,
    ymultiplier: usize,
    screen: [u8; WIDTH * HEIGHT],
    buffer: Vec<u32>,
}

impl Display {
    pub fn new(scalingx: usize, scalingy: usize) -> Display {
        Display {
            xmultiplier: (scalingx),
            ymultiplier: (scalingy),
            screen: [0; WIDTH * HEIGHT],
            buffer: Vec::new(),
        }
    }

    pub fn getscreen(&self) -> [u8; WIDTH * HEIGHT] {
        self.screen
    }

    pub fn getbuffer_as_u32(self) -> Vec<u32> {
        //self.buffer
        todo!()
    }
}
