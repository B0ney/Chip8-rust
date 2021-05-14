
use crate::video;
//use keypad;
use std::fs::File;
use std::io::prelude::*;
use rand::random;
const STARTADDR: u16 = 0x200; // Memory map from 0x200 to 0xfff used to stor rom and work ram

static CHIP8_FONTSET: [u8;80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F 
];

pub struct CPU {

    pub memory: [u8; 4096],
    pub display: video::Display,
    pub pc:     usize,            // program counter
    pub opcode: u16,
    pub v:      [u8; 16],       // V registers
    pub stack:  [usize; 16],
    pub i:      usize,            // index register
    pub sp:     usize,             // stack pointer
    
    pub dt:     u8,             // delay timer

}

impl CPU {
    pub fn new() -> CPU {
        let mut new_cpu = CPU {
            // initialises memory, registers and video buffers
            memory:     [0u8; 4096],
            display:    video::Display::new(), 
            pc:         STARTADDR as usize,
            opcode:     0,
            v:          [0u8; 16],
            stack:      [0; 16],
            i:          STARTADDR as usize, 
            sp:         0, 
            dt:         0,             
        };
        //load fontset
        for i in 0..80 {
            new_cpu.memory[i] = CHIP8_FONTSET[i]
        };

        return new_cpu;
    }
    pub fn load_rom(&mut self, path: &str) {
        let mut f = File::open(path).expect("file does not exist");        
        let mut buffer = [0u8;3584];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

        for (i, &byte) in buffer.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[addr] = byte;
            } else {
                break;
            }
        }
        println!("{:x?}", self.memory)

    }
    pub fn emulate_cycle(&mut self) {
        // fetch
        // decode 
        // execute
        self.fetch_opcode();
        self.execute();
        println!("Opcode: {:x}", self.opcode);
        println!("Program counter: {:x}", self.pc);


        //update timer
    }

    pub fn fetch_opcode(&mut self) {
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
            0x0000  => self.op_0xxx(),
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
            _ => self.opcode_not_found(self.opcode),
        }
    }

    fn op_0xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => self.display.clear(), // Clears the screen. 
            0x000E => { //Returns from a subroutine. 
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
                },
            _ => self.opcode_not_found(self.opcode),
        }
        self.pc += 2;

    }

    fn op_1xxx(&mut self) {
        //Jumps to address NNN.
        self.pc = self.op_nnn() as usize;
    }

    fn op_2xxx(&mut self) {
        // Calls subroutine at NNN. 
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.op_nnn() as usize;
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
        // Sets VX to NN.
        self.v[self.op_x()] = self.op_nn();
        self.pc += 2; 
    }

    fn op_7xxx(&mut self) {
        // Adds NN to VX.
        println!("{:x?}", self.v);
        println!("{:x}", self.opcode);
        println!("{:x}",self.op_nn());
        let vx = self.v[self.op_x()] as u16;
        let val = self.op_nn() as u16;
        let result = vx + val;

        self.v[self.op_x()] = result as u8;
        self.pc += 2; 
    }

    fn op_8xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => { self.v[self.op_x()] = self.v[self.op_y()]; }, // Sets VX to the value of VY. 
            0x0001 => { self.v[self.op_x()] |= self.v[self.op_y()]; }, // Sets VX to VX or VY. 
            0x0002 => { self.v[self.op_x()] &= self.v[self.op_y()]; }, // Sets VX to VX and VY.
            0x0003 => { self.v[self.op_x()] ^= self.v[self.op_y()]; }, // Sets VX to VX xor VY. 
            0x0004 => { //Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not. 
                // I had issues with overflow errors in this section
                // credit to https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs
                // as this helped solve this issue
                let vx = self.v[self.op_x()]  as u16;
                let vy = self.v[self.op_y()]  as u16;
                let result = vx + vy;

                self.v[self.op_x()] = result as u8;
                self.v[0x0F] = if result > 0xFF {1} else {0};

            },
            0x0005 => {
                //VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not. 
                self.v[0x0F] = if self.v[self.op_x()] > self.v[self.op_y()] {1} else {0};
                self.v[self.op_x()] = self.v[self.op_x()].wrapping_sub(self.v[self.op_y()]);
                
            },
            0x0006 => {
                //Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                self.v[0xF] = self.v[self.op_x()] & 0x1 ;
                self.v[self.op_x()] >>= 1;
            },
            0x0007 => {
                //Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not. 
                self.v[self.op_x()] = self.v[self.op_y()] - self.v[self.op_x()];

                if self.v[self.op_x()] > self.v[self.op_y()] {
                    self.v[0xF] = 0

                } else {
                    self.v[0xF] = 1
                };
            },
            0x000E => {
                //Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                // please check
                self.v[0xF] = self.v[self.op_x()] >> 7; // shift 8 bits to right 7 places to get msb
                self.v[self.op_x()] <<= 1;
            },
            _ => self.opcode_not_found(self.opcode),
        }
        self.pc += 2;
        println!("section works!");
    }

    fn op_9xxx(&mut self) {
        if self.v[self.op_x()] != self.v[self.op_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn op_Axxx(&mut self) {
        self.i = self.op_nnn() as usize;
        self.pc += 2;
    }
    fn op_Bxxx(&mut self) {
        //Jumps to the address NNN plus V0. 
        self.pc = ((self.v[0] as u16) + self.op_nnn()) as usize; 
    }
    fn op_Cxxx(&mut self) {
        //Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN. 
        self.v[self.op_x()] = random::<u8>() & self.op_nn();
        self.pc += 2;
    }
    fn op_Dxxx(&mut self) {
        // TODO
        let x = self.v[self.op_x()] as usize;
        let y = self.v[self.op_y()] as usize;
        //let height = self.opcode & 0x000F;

        let start_slice = self.i as usize;
        let end_slice = start_slice + (self.op_n() as u16) as usize ;

        let sprite = &self.memory[start_slice..end_slice];

        self.v[0xF] = self.display.draw(x, y, sprite); // draw function returns collision data
        self.pc += 2;

    }

    fn op_Exxx(&mut self) {
        // io not implemented, so assume key not pressed down
        match self.opcode & 0x00FF {
            0x009E =>{self.pc += 2},//
            0x00A1 => {self.pc += 4},
            _ => self.opcode_not_found(self.opcode),
        }        
        
    }
    fn op_Fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x0007 => { self.v[self.op_x()] = self.dt },
            0x000A => self.opcode_not_found(self.opcode),
            0x001E => { self.i += self.v[self.op_x()] as usize },
            0x0029 => { self.i = (self.v[self.op_x()] as usize) * 5 },
            0x0033 => {
                self.memory[self.i] = self.v[self.op_x()] / 100;
                self.memory[self.i + 1] = (self.v[self.op_x()] % 100) / 10;
                self.memory[self.i + 2] = self.v[self.op_x()] % 10;
            }
            0x0055 => {
                for i in 0..=self.op_x() {
                    self.memory[self.i + i] = self.v[i];
                }
            }
            0x0065 => {
                for i in 0..=self.op_x() {
                    self.v[i] = self.memory[self.i + i];
                }
            },
            _ => self.opcode_not_found(self.opcode),
        }
        self.pc += 2;
    }

    fn opcode_not_found(&mut self, opcode: u16) {
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