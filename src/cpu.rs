use rand;

pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    rng: rand::rngs::ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            i: (0x0),
            pc: (0x200),
            stack: [0; 16],
            sp: (0x0),
            rng: (rand::rngs::ThreadRng::default()),
        }
    }

    //run()
    /*
       0nnn
       00e0
       00ee
       1nnn
       2nnn
       3xkk
       4xkk
       5xy0
       6xkk
       7xkk
       8xy0
       8xy1
       8xy2
       8xy3
       8xy4
       8xy5
       8xy6
       8xy7
       8xyE
       9xy0
       Annn
       Bnnn
       Cxkk
       Dxyn
       Ex9E
       ExA1
       Fx07
       Fx0A
       Fx15
       Fx18
       Fx1E
       Fx29
       Fx33
       Fx55
       Fx65
    */
}
