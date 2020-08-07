use crate::ram::Ram;
use std::error::Error;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: Vec<u8>,
    pc: u16,
    prev_pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vx: vec![0; 16],
            pc: PROGRAM_START,
            prev_pc: 0,
            i: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) -> Result<(), Box<dyn Error>> {
        let high = ram.read_byte(self.pc as usize)? as u16;
        let low = ram.read_byte((self.pc + 1) as usize)? as u16;
        let instruction: u16 = (high << 8) | low;
        println!("Instruction read : {:#X?}, high: {:#X?}, low: {:#X?}", instruction, high, low);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        println!("nnn: {:X?}, nn: {:X?}, n: {:X?}, x: {:X?}, y: {:X?}", nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("Please increment PC");
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x0 => {

            },
            0x1 => {
                // goto NNN
                self.pc = nnn;
            },
            0x3 => {
                // if(Vx==NN)
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // Vx = NN
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                // Vx += NN
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            },
            0xA => {
                // I = NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xD => {
                // draw(Vx,Vy,N)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.draw_sprite(ram, vx, vy, n);
                self.pc += 2;
            },
            0xF => {
                match nn {
                    0x1E => {
                        // I += Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    }
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    }
                }
            }
            _ => {
                return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
            },
        }

        Ok(())
    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    pub fn read_reg_vx(&self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    fn draw_sprite(&self, ram: &mut Ram, vx: u8, vy: u8, height: u8) {
        println!(
            "[DEBUG] Drawing a sprite at ({}, {}), of width {} and height {}",
            vx,
            vy,
            8,
            height,
        );
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpu {{\n");
        write!(f, "\tvx = {:X?}\n", self.vx);
        write!(f, "\tpc = {:X?}\n", self.pc);
        write!(f, "\ti = {:X?}\n", self.i);
        write!(f, "}}")
    }
}