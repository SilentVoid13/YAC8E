use crate::bus::Bus;

use std::error::Error;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;

/// Cpu emulation
pub struct Cpu {
    /// 16 8-bit registers
    vx: Vec<u8>,
    /// Program Counter (or Instruction Pointer)
    pc: u16,
    // TODO: Remove this
    prev_pc: u16,
    /// 16-bit register used to store memory addresses,
    i: u16,
    /// Stack Containing 16 16-bit values at maximum
    /// Only used for return addresses in CHIP-8
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vx: vec![0; 16],
            pc: PROGRAM_START,
            prev_pc: 0,
            i: 0,
            stack: vec![],
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) -> Result<(), Box<dyn Error>> {
        let high = bus.ram.read_byte(self.pc as usize)? as u16;
        let low = bus.ram.read_byte((self.pc + 1) as usize)? as u16;
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
            0x2 => {
                // Call subroutine at address NNN
                // *(0xNNN)()
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            0x3 => {
                // if(Vx==NN)
                let vx = self.read_reg_vx(x);
                self.skip_if(vx == nn);
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
            0x8 => {
                match n {
                    0x0 => {
                        // Vx=Vy
                        self.write_reg_vx(x, self.read_reg_vx(y));
                    },
                    0x1 => {

                    },
                    0x2 => {

                    },
                    0x3 => {

                    },
                    0x4 => {

                    },
                    0x5 => {

                    },
                    0x6 => {

                    },
                    0x7 => {

                    },
                    0xE => {

                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    }
                }
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
                bus.display.draw_sprite(vx, vy, n);
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0x9E => {
                        // if(key()==Vx)
                        let vx = self.read_reg_vx(x);
                        self.skip_if(bus.keyboard.key_pressed == vx);
                    },
                    0xA1 => {
                        // 	if(key()!=Vx)
                        let vx = self.read_reg_vx(x);
                        self.skip_if(bus.keyboard.key_pressed != vx);
                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    },
                }
            }
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

    pub fn skip_if(&mut self, cond: bool) {
        if cond {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
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