mod cpu {
    const DISPLAY_WIDTH: usize = 32;
    const DISPLAY_HEIGHT: usize = 64;
    const RAM_SIZE: usize = 4096;
    use rand::Rng;

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

    enum Opcode {
        // SYS,    // Ignore? syscall
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
        ADD_SP, // Add sprite
        LD_B,   // Load BCD value of reg into memory at I, I+1, I+2
        LD_MUL, // Load V0 to Vx into memory starting at I
        LDR_MUL,// Load V0 to Vx with memory starting at I
        UNKNOWN,// Unknown opcode
    }

    #[derive(Copy, Clone)]
    struct Operands {
        instruction: u16
    }

    impl Operands {
        fn new(instruction: u16) -> Operands {
            Operands {
                instruction: instruction
            }
        }

        fn nnn(self) -> u16 {
            self.instruction & 0x0FFF // Lowest 12 bits
        }

        fn nn(self) -> u16 {
            self.instruction & 0x00FF // Lowest 8 bits
        }

        fn n(self) -> u16 {
            self.instruction & 0x000F // Lowest 4 bits
        }

        fn x(self) -> usize {
            ((self.instruction >> 8) & 0x000F).into() // Lower 4 of high byte
        }

        fn y(self) -> usize {
            ((self.instruction >> 4) & 0x000F).into() // Upper 4 of lower byte
        }

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
           let (opcode, operands) = Cpu::decode(instruction);

           // Execute the instruction
           self.execute(opcode, operands);

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

        fn decode(instruction : u16) -> (Opcode, Operands) {
            // Get operands first so can use to find opcode
            let operands = Operands::new(instruction);

            // Get opcode
            let opcode = (instruction >> 12) as u8;
            let opcode: Opcode = match opcode {
                0 => match operands.nn() {
                    0xE0 => Opcode::CLS,
                    0xEE => Opcode::RET,
                    _    => Opcode::UNKNOWN,
                }
                1 => Opcode::JP,
                2 => Opcode::CALL,
                3 => Opcode::SE_IMM,
                4 => Opcode::SNE_IMM,
                5 => Opcode::SE,
                6 => Opcode::LDR_IM,
                7 => Opcode::ADD_IM,
                8 => match operands.n() {
                    0x0 => Opcode::LDR,
                    0x1 => Opcode::OR,
                    0x2 => Opcode::AND,
                    0x3 => Opcode::XOR,
                    0x4 => Opcode::ADD,
                    0x5 => Opcode::SUB,
                    0x6 => Opcode::SHR,
                    0x7 => Opcode::SUBN,
                    0xE => Opcode::SHL,
                    _   => Opcode::UNKNOWN,
                }
                9 => Opcode::SNE,
                0xA => Opcode::LD_I,
                0xB => Opcode::JP_REG,
                0xC => Opcode::RND,
                0xD => Opcode::DISPLAY,
                0xE => match operands.nn() {
                    0x9E => Opcode::SKP,
                    0xA1 => Opcode::SKNP,
                    _    => Opcode::UNKNOWN,
                }
                0xF => match operands.nn() {
                    0x07 => Opcode::LDR_DT,
                    0x0A => Opcode::LDR_KP,
                    0x15 => Opcode::LD_DT,
                    0x18 => Opcode::LD_ST,
                    0x1E => Opcode::ADD_I,
                    0x29 => Opcode::ADD_SP,
                    0x33 => Opcode::LD_B,
                    0x55 => Opcode::LD_MUL,
                    0x65 => Opcode::LDR_MUL,
                    _    => Opcode::UNKNOWN,
                }
                _ => Opcode::UNKNOWN,
            };

            (opcode, operands)
        }

        fn execute(&mut self, opcode: Opcode, operands: Operands) {
                match opcode {
                    Opcode::CLS     => self.cls(),
                    Opcode::RET     => self.ret(),
                    Opcode::JP      => self.jp(operands),
                    Opcode::CALL    => self.call(operands),
                    Opcode::SE_IMM  => self.se_imm(operands),
                    Opcode::SNE_IMM => self.sne_imm(operands),
                    Opcode::SE      => self.se(operands),
                    Opcode::LDR_IM  => self.ldr_im(operands),
                    Opcode::ADD_IM  => self.add_im(operands),
                    Opcode::LDR     => self.ldr(operands),
                    Opcode::OR      => self.or(operands),
                    Opcode::AND     => self.and(operands),
                    Opcode::XOR     => self.xor(operands),
                    Opcode::ADD     => self.add(operands),
                    Opcode::SUB     => self.sub(operands),
                    Opcode::SHR     => self.shr(operands),
                    Opcode::SUBN    => self.subn(operands),
                    Opcode::SHL     => self.shl(operands),
                    Opcode::SNE     => self.sne(operands),
                    Opcode::LD_I    => self.ld_i(operands),
                    Opcode::JP_REG  => self.jp_reg(operands),
                    Opcode::RND     => self.rnd(operands),
                    Opcode::DISPLAY => self.display(operands),
                    Opcode::SKP     => self.skp(operands),
                    Opcode::SKNP    => self.sknp(operands),
                    Opcode::LDR_DT  => self.ldr_dt(operands),
                    Opcode::LDR_KP  => self.ldr_kp(operands),
                    Opcode::LD_DT   => self.ld_dt(operands),
                    Opcode::LD_ST   => self.ld_st(operands),
                    Opcode::ADD_I   => self.add_i(operands),
                    Opcode::ADD_SP  => self.add_sp(operands),
                    Opcode::LD_B    => self.ld_b(operands),
                    Opcode::LD_MUL  => self.ld_mul(operands),
                    Opcode::LDR_MUL => self.ldr_mul(operands),
                    Opcode::UNKNOWN => println!("Unknown opcode"),
                }
            
        }

        fn update_timers(&mut self) {
            // To be implemented..
        }
        
        fn cls(&mut self){

        }

        fn ret(&mut self){
            // Set pc to address at top of the stack
            self.pc -= 1;
        }

        fn jp(&mut self, operands: Operands){
            self.pc = operands.nnn();
        }

        fn call(&mut self, operands: Operands){
            self.sp += 1;
            // Put pc to top of stack
            self.pc = operands.nnn();
        }

        fn se_imm(&mut self, operands: Operands){
            if self.v[operands.x()] == operands.nn() as u8 {
                self.pc += 2
            }
        }

        fn sne_imm(&mut self, operands: Operands){
            if self.v[operands.x()] != operands.nn() as u8 {
                self.pc += 2
            }
        }

        fn se(&mut self, operands: Operands){
            if self.v[operands.x()] == self.v[operands.y()] {
                self.pc += 2
            }
        }

        fn ldr_im(&mut self, operands: Operands){
            self.v[operands.x()] = operands.nn() as u8;
            self.pc += 2
        }

        fn add_im(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.x()] + operands.nn() as u8;
            self.pc += 2
        }

        fn ldr(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.y()];
            self.pc += 2
        }

        fn or(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.x()] | self.v[operands.y()];
            self.pc += 2
        }

        fn and(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.x()] & self.v[operands.y()];
            self.pc += 2
        }

        fn xor(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.x()] ^ self.v[operands.y()];
            self.pc += 2
        }

        fn add(&mut self, operands: Operands){
            let x = self.v[operands.x()];
            let y = self.v[operands.y()];
            let result = x + y;

            self.v[0x0F] = if result > 0xFF { 1 } else { 0 };
        }

        fn sub(&mut self, operands: Operands){
            let x = self.v[operands.x()];
            let y = self.v[operands.y()];

            self.v[0x0F] = if x > y { 1 } else { 0 };
            self.v[operands.x()] = self.v[operands.x()] - self.v[operands.y()]
        }

        fn shr(&mut self, operands: Operands){
            self.v[0x0F] = if self.v[operands.x()] & 0x1 == 1 { 1 } else { 0 };
            self.v[operands.x()] = self.v[operands.x()] / 2
        }

        fn subn(&mut self, operands: Operands){
            self.v[0x0F] = if self.v[operands.y()] > self.v[operands.x()] { 1 } else { 0 };
            self.v[operands.x()] = self.v[operands.y()] - self.v[operands.x()]
        }
        
        fn shl(&mut self, operands: Operands){
            self.v[0x0F] = if self.v[operands.x()] >> 7 == 1 { 1 } else { 0 };
            self.v[operands.x()] = self.v[operands.x()] * 2
        }

        fn sne(&mut self, operands: Operands){
            if self.v[operands.x()] != self.v[operands.y()]{
                self.pc += 2}
        }

        fn ld_i(&mut self, operands: Operands){
            self.i = operands.nnn()
        }

        fn jp_reg(&mut self, operands: Operands){
            self.pc = self.v[0] as u16 + operands.n();
        }
        
        fn rnd(&mut self, operands: Operands){
            let rnd_num: u8 = rand::thread_rng().gen_range(0, 255);
            let and = rnd_num & (operands.nn() as u8);

            self.v[operands.x()] = and
        }
        
        fn display(&mut self, operands: Operands){
            // display sprite
        }
        
        fn skp(&mut self, operands: Operands){
            // skip if key pressed
        }
        
        fn sknp(&mut self, operands: Operands){
            // skip if no key pressed
        }
        
        fn ldr_dt(&mut self, operands: Operands){
            self.v[operands.x()] = self.v[operands.x()] + self.delay_timer
        }
        
        fn ldr_kp(&mut self, operands: Operands){
            // Wait for keypress and load reg
        }
        
        fn ld_dt(&mut self, operands: Operands){
            self.v[operands.x()] = self.delay_timer
        }
        
        fn ld_st(&mut self, operands: Operands){
            self.v[operands.x()] = self.sound_timer
        }
        
        fn add_i(&mut self, operands: Operands){
            self.i = self.i + self.v[operands.x()] as u16
        }

        fn add_sp(&mut self, operands: Operands) {
            // add sprite
        }
        
        fn ld_b(&mut self, operands: Operands){
            self.ram[self.i]  
            self.ram[self.i + 1] 
            self.ram[self.i + 2] 

            // need to look into bcd 
        }
        
        fn ld_mul(&mut self, operands: Operands){
        }
        
        fn ldr_mul(&mut self, operands: Operands){
        }
    }
}
