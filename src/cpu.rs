mod video::Display;
const STARTADDR: u16 = 0x200; // Memory map from 0x200 to 0xfff used to stor rom and work ram

static CHIP8_FONTSET: [u8;80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80 
];

pub struct CPU {

    pub memory: [u8; 4096],
    pub display: video::Display,
    pub pc:     u16,            // program counter
    pub opcode: u16,
    pub v:      [u8; 16],       // V registers
    pub stack:  [u16; 16],
    pub i:      u16,            // index register
    pub sp:     u8,             // stack pointer
    
    pub dt:     u8,             // delay timer

}

impl CPU {
    pub fn new() -> CPU {
        let mut new_cpu = CPU {
            // initialises memory, registers and video buffers
            memory:     [0u8; 4096],
            display:    video::Display::new(), 
            pc:         STARTADDR,
            opcode:     0,
            v:          [0u8; 16],
            stack       [0; 16],
            i:          pc,
            sp:         0, 
            dt:         0,             
        };
        //load fontset
        for _ in 0..=80 {
            new_cpu.memory[_] = CHIP8_FONTSET[_]
        };

        return new_cpu;
    }
    pub fn emulate_cycle(&mut self) {
        // fetch
        // decode 
        // execute
        self.fetch_opcode();
        self.execute();


        //update timer
    }

    pub fn fetch_opcode(&self) -> u16 {
        /*  Example
            memory[0x200] = 11010110  <- 8 bits
            memory[0x201] = 10100101  <- 8 bits

            memory[0x200] << 8 = 1101011000000000  <-shift first instrucion by 8 bits

            memory[0x200] << 8 | memory[0x201] = 1101011000000000  <- Logical OR bit instructions to get 16 bits 
                                                         10100101
                                                         
                                                 1101011010100101 

        */
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);


    }

    pub fn execute(&mut self) {
        /* 
            each instruction is two bytes long 
            so program counter increments by 2 since memory is array of u8
            https://en.wikipedia.org/wiki/CHIP-8#Opcode_table

        */
        match (self.opcode & 0xf000) {
            0x0000  => self.op_0xxx(),//self.display.clear(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_Axxx(),
            0xB000 => self.op_Bxxx(),
            0xC000 => self.op_Cxxx(),
            0xD000 => self.op_Dxxx(),
            0xE000 => self.op_Exxx(),
            0xF000 => self.op_Fxxx(),
            _ => opcode_not_found(self.opcode),
        }
    }
    pub fn draw() {

    }

    fn op_0xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => self.display::clear(), // Clears the screen. 
            0x000E => { //Returns from a subroutine. 
                self.sp -= 1;
                self.pc = self.stack[self.sp] as u16;
                },
            _ => opcode_not_found(self.opcode),
        }
        self.pc += 2;

    }
    fn op_1xxx(&mut self) {
        //Jumps to address NNN.

        self.pc = self.op_nnn();
    }
    fn op_2xxx(&mut self) {
        // Calls subroutine at NNN. 

        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.op_nnn();
    }

    fn op_3xxx(&mut self) {
        //Skips the next instruction if VX equals NN.

        if self.v[self.op_x()] == self.op_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_4xxx(&mut self) {
        //Skips the next instruction if VX does not equal NN. 
        if self.v[self.op_x()] != self.op_nn(){
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_5xxx(&mut self) {
        // Skips the next instruction if VX equals VY. 
        if self.v[self.op_x()] == self.v[self.op_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_6xxx(&mut self) {
        //Sets VX to NN.
        self.v[self.op_x()] = self.op_nn();
        self.pc += 2; 
    }

    fn op_7xxx(&mut self) {
        //Adds NN to VX.
        self.v[self.op_x()] += self.op_nn();
        self.pc += 2; 
    }

    fn op_8xxx(&mut self) {
        match (self.opcode & 0x000F) {

        }
    }

    fn op_9xxx(&mut self) {
        
    }

    fn op_Axxx(&mut self) {
        
    }
    fn op_Bxxx(&mut self) {
        
    }
    fn op_Cxxx(&mut self) {
        
    }
    fn op_Dxxx(&mut self) {
        
    }
    fn op_Exxx(&mut self) {
        
    }
    fn op_Fxxx(&mut self) {
        
    }
    fn opcode_not_found(opcode: u16) {
        println!("OPCODE {:x} Not implemented", opcode );
    }

    fn op_nnn(&self)    -> u16 { (self.opcode & 0x0FFF) } // 16 bit address
    fn op_nn(&self)     -> u8 { (self.opcode & 0x00FF) as u8 } // 8 bit constant
    fn op_n(&self)      -> u8 { (self.opcode & 0x000F) as u8 } // "4" bit constant

    // 4 bit register identifier
    fn op_x(&self) -> usize { ((self.opcode & 0x0F00) >> 8) as usize} // remove trailing 8 bits
    fn op_y(&self) -> usize { ((self.opcode & 0x00F0) >> 4) as usize} // remove trailing 4 bits









}

#[test]
fn test1(){
    let cpu = CPU::new();
    cpu.fetch_opcode()
}