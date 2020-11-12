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
        SE_IMM, // Conditional skip
        SNE_IMM,// Conditional not skip
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
            let instruction = self.ram[pc] << 8 | self.ram[pc];
            instruction as u16
        }

        // TODO: Actually return Opcode enum
        fn decode(instruction : u16) -> (u8, u16) {
            let opcode = (instruction >> 12) as u8;
            let arguments = instruction & 0x0FFF;

            (opcode, arguments)
        }

        fn execute(&mut self, opcode: u8, instruction: u16) {
            // To be implemented..
        }

        fn update_timers(&mut self) {
            // To be implemented..
        }
        
    }
}