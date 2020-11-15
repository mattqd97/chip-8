mod cpu {
    const DISPLAY_WIDTH: usize = 32;
    const DISPLAY_HEIGHT: usize = 64;
    const RAM_SIZE: usize = 4096;

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
        display: [u8; DISPLAY_HEIGHT * DISPLAY_WIDTH], // 64 * 32 8 bit values
    }

    enum Opcodes {
        SYS,    // Ignore? syscall
        CLS,    // Clear Screen
        RET,    // Return from subroutine
        JP,     // Jump to addr
        CALL,   // Call subroutine
        SE_IMM, // Conditional skip 3xkk
        SNE_IMM,// Conditional not skip
        SE,     // Compare register
        LDR_IM, // Load immediate into reg
        ADD_IM, // Add immediate to reg
        LDR,    // Load reg into another reg
        OR,     // Bitwise OR two regs
        AND,    // AND 2 regs
        XOR,    // XOR 2 regs
        ADD,    // Add 2 regs
        SUB,    // Subtract 2 regs
        SHR,    // Shift right by 1
        SUBN,   // SUB, but different
        SHL,    // Shift left by 1
        SNE,    // Conditional skip
        LD_I,   // Load I register
        JP_REG, // Jump to reg + offset
        RND,    // Random byte AND immediate
        DISPLAY,// Display sprite
        SKP,    // Skip next instruction key pressed
        SKNP,   // Skip if key not pressed
        LDR_DT, // Load reg with delay timer
        LDR_KP, // Wait for keypress and load reg
        LD_DT,  // Load delay timer with reg
        LD_ST,  // Load sound timer with reg
        ADD_I,  // Add value to I reg
        LD_B,   // Load BCD value of reg into memory at I, I+1, I+2
        LD_MUL, // Load V0 to Vx into memory starting at I
        LDR_MUL,// Load V0 to Vx with memory starting at I
    }

    impl Cpu {
        // Initial setup.
        pub fn new(&mut self) {
            self.v = [0; 16];
            self.i = 0;
            self.stack = [0; 16];
            self.pc = 0x200;
            self.sp = 0;
            self.delay_timer = 0;
            self.sound_timer = 0;
            self.ram = [0; RAM_SIZE];
            self.keypad = [false; 16];
            self.display = [0; DISPLAY_HEIGHT * DISPLAY_WIDTH];
        }

        // This is the core function, which will simulate a cycle
        pub fn simulate_cycle(&mut self) {
           // Fetch the next instruction
           let instruction = self.fetch();

           // Decode the instruction
           let (opcode, arguments) = Cpu::decode(instruction);

           // Execute the instruction
           self.execute(opcode, arguments);

           // Update Timers
           self.update_timers();

           // Update PC
           self.pc += 2;
        }

        fn fetch(&self) -> u16 {
            let pc = self.pc as usize;
            let instruction = self.ram[pc] << 8 | self.ram[pc + 1];
            instruction as u16
        }

        fn decode(instruction : u16) -> (u8, u16) {
            let opcode = (instruction >> 12) as u8;
            let nnn = instruction << 4 as u16; // Lowest 12 bits
            let n = (instruction & 0x000F) as u8; // Lowest 4 bits
            let x = (instruction >> 8) as u8 & 0x000F; // Lower 4 of high byte
            let y = (instruction & 0xF000) as u8; // Upper 4 of lowe byte
            let kk = (instruction & 0x00FF) as u8; // lowest 8 bits

            // Do we even need to shift anything? 
            // Would let x = instruction & 0x000F do the same thing?

            (opcode, nnn) // What does this do?
        }

        fn execute(&mut self, opcode: u8, nnn: u16, x: u8, y: u8, n: u8, kk: u8) {
                match opcode {
                    1 => self.jp(),
                    2 => self.call(),
                    3 => self.se_imm(),
                    4 => self.sne_imm(),
                    5 => self.se(),
                    6 => self.ldr_im(),
                    7 => self.add_im(),
                    8 => 
                    match n {
                        0 => self.ldr(),
                        1 => self.or(),
                        2 => self.and(),
                        3 => self.xor(),
                        4 => self.add(),
                        5 => self.sub(),
                        6 => self.shr(),
                        7 => self.subn(),
                        174 => self.shl(),
                    }
                    9 => self.sne(),
                    10 => self.ld_i(),
                    11 => self.jp_reg(),
                    12 => self.rnd(),
                    13 => self.display(),
                    14 => 
                    match n {
                        1 => self.sknp(),
                        14 => self.skp(),
                    }
                    15 => 
                    match n {
                        3 => self.ld_b(),
                        5 =>
                        match y {
                            1 => self.ldr_mul(),
                            5 => self.ld_mul(),
                            6 => self.ld_dt(),
                        }
                        7 => self.ldr_dt(),
                        8 => self.ld_st(),
                        9 => self.ld_i(),
                        10 => self.ldr_kp(),
                        14 => self.add_i(),
                    }
                }
            
        }

        fn update_timers(&mut self) {
            // To be implemented..
        }
        
        fn cls(&mut self){

        }

        fn ret(&mut self){

        }

        fn jp(&mut self){

        }

        fn call(&mut self){

        }

        fn se_imm(&mut self){

        }

        fn sne_imm(&mut self){

        }

        fn se(&mut self){

        }

        fn ldr_im(&mut self){

        }

        fn add_im(&mut self){

        }

        fn ldr(&mut self){

        }

        fn or(&mut self){

        }

        fn and(&mut self){

        }

        fn xor(&mut self){

        }

        fn add(&mut self){

        }

        fn sub(&mut self){

        }

        fn shr(&mut self){

        }

        fn subn(&mut self){

        }
        
        fn shl(&mut self){

        }

        fn sne(&mut self){

        }

        fn ld_i(&mut self){

        }

        fn jp_reg(&mut self){

        }
        
        fn rnd(&mut self){

        }
        
        fn display(&mut self){

        }
        
        fn skp(&mut self){

        }
        
        fn sknp(&mut self){

        }
        
        fn ldr_dt(&mut self){

        }
        
        fn ldr_kp(&mut self){

        }
        
        fn ld_dt(&mut self){

        }
        
        fn ld_st(&mut self){

        }
        
        fn add_i(&mut self){

        }
        
        fn ld_b(&mut self){

        }
        
        fn ld_mul(&mut self){

        }
        
        fn ldr_mul(&mut self){

        }
    }
}