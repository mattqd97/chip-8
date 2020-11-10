const DISPLAY_WIDTH: u8 = 32;
const DISPLAY_HEIGHT: u8 = 64;
const RAM_SIZE: u8 = 4096;

pub struct Cpu {
    v: [u8; 16], // Creates 16 v variables with type u8.
    i: u16, // 16 bit index register.
    stack: [u16; 16], // 16, 16 bit values.
    pc: u16, // 16 bit Program counter.
    sp: u8, // 8 bit Stack pointer.
    delay_timer: u8, // 8 Bit delay timer.
    sound_timer: u8, // 8 Bit sound timer.
    ram: [u8; RAM_SIZE], // 4096 8 bit values to total 4k RAM?
    keypad: [bool; 16], // Keypad has 16 keys
    display_height: [u8; DISPLAY_HEIGHT], // 64 8 bit values
    display_width: [u8; DISPLAY_WIDTH] // 32 8 bit values
}
