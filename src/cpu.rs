mod bus;


const DISPLAY_WIDTH: u8 = 32;
const DISPLAY_HEIGHT: u8 = 64;

pub struct Cpu {
    v: [u8; 16], // Creates 16 v variables with type u8.
    i: u16, // 16 bit index register.
    stack: [u16; 16], // 16, 16 bit values.
    pc: u16, // 16 bit Program counter.
    sp: u8, // 8 bit Stack pointer.
    delay_timer: u8, // 8 Bit delay timer.
    sound_timer: u8, // 8 Bit sound timer.
    ram: [u8; RAM_SIZE], // 4096 8 bit values to total 4k RAM?
    keypad: [bool; 16], // Keypad has 16 keys.
    screen: [[u8; DISPLAY_HEIGHT]; [DISPLAY_WIDTH]] // Sets screen size.
}


impl Cpu {
    // Implement the cpu and assign values to registers.
    pub fn new() -> Self { 
        let mut ram = [u8; RAM_SIZE];


    Cpu {
    v: [0; 16], // Set all registers to zero.
    i: 0, // Set index register to 0.
    stack: [0; 16], // Sets stack to 0. 
    pc: 0x200, // Set program counter to start at 0x200.
    sp: 0, // Set stack pointer to start to 0, top of the stack.
    delay_timer: 0, // Set delay timer to 0.
    sound_timer: 0, // Set timer to 0.
    ram: ram, // Sets ram as RAM_SIZE, maybe so ram size can be increased?
    keypad: [false; 16], // No key has been pressed.
    screen: [[0; DISPLAY_HEIGHT]; [DISPLAY_WIDTH]] // Blank screen.
        }
}
}