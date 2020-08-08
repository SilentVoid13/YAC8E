use crate::bus::Bus;
use crate::utils::{log_debug, log_warning, log_special};

use std::error::Error;
use std::fmt;

use rand::Rng;

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
        log_debug(
            format!(
                "Instruction read : {:#X?}, high: {:#X?}, low: {:#X?}", instruction, high, low
            )
        );

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        log_debug(
            format!(
                "nnn: {:X?}, nn: {:X?}, n: {:X?}, x: {:X?}, y: {:X?}", nnn, nn, n, x, y
            )
        );

        if self.prev_pc == self.pc {
            panic!("Please increment PC");
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                // We don't implement opcode 0x0NNN, only used on older machines
                match nn {
                    0xE0 => {
                        bus.display.clear();
                        self.pc += 2;
                    },
                    0xEE => {
                        //self.pc = self.stack.pop().ok_or("Impossible instruction: 0x00EE, no return address on stack")?;
                        //return from subroutine
                        let addr = self.stack.pop().unwrap();
                        self.pc = addr;
                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    },
                }
            },
            0x1 => {
                // goto NNN
                self.pc = nnn;
            },
            0x2 => {
                // Call subroutine at address NNN
                // *(0xNNN)()
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            },
            0x3 => {
                // if(Vx==NN)
                let vx = self.read_reg_vx(x);
                self.skip_if(vx == nn);
            },
            0x4 => {
                // 	if(Vx!=NN)
                let vx = self.read_reg_vx(x);
                self.skip_if(vx != nn);
            },
            0x5 => {
                // if(Vx==Vy)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.skip_if(vx == vy)
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
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                match n {
                    0x0 => {
                        // Vx=Vy
                        self.write_reg_vx(x, vy);
                    },
                    0x1 => {
                        panic!();
                        // Vx=Vx|Vy
                        //self.write_reg_vx(x, vx | vy);
                    },
                    0x2 => {
                        // Vx=Vx&Vy
                        self.write_reg_vx(x, vx & vy);
                    },
                    0x3 => {
                        // 	Vx=Vx^Vy
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    0x4 => {
                        // Vx += Vy
                        let sum = vx as u16 + vy as u16;
                        self.write_reg_vx(x, (sum % 256) as u8);
                        if sum > 0xFF {
                            log_warning(
                                format!(
                                    "Overflow detected on instruction: {:X}", instruction
                                )
                            );
                            self.write_reg_vx(0xF, 1);
                        }
                        else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    0x5 => {
                        // Vx -= Vy
                        if vx > vy {
                            self.write_reg_vx(0xF, 1);
                        }
                        else {
                            self.write_reg_vx(0xF, 0);
                        }
                        self.write_reg_vx(x, vx - vy);
                    },
                    0x6 => {
                        // Vx>>=1
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    },
                    0x7 => {
                        // Vx=Vy-Vx
                        if vy > vx {
                            self.write_reg_vx(0xF, 1);
                        }
                        else {
                            self.write_reg_vx(0xF, 0);
                        }
                        self.write_reg_vx(x, vy - vx);
                    },
                    0xE => {
                        // Vx<<=1
                        self.write_reg_vx(0xF, vx >> 7);
                        self.write_reg_vx(x, vx << 1);
                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    }
                }
                self.pc += 2;
            },
            0x9 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.skip_if(vx != vy);
            },
            0xA => {
                // I = NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xB => {
                // PC=V0+NNN
                self.pc = self.read_reg_vx(0) as u16 + nnn;
            },
            0xC => {
                let mut rng = rand::thread_rng();
                let num: u8 = rng.gen_range(0, 255);
                self.write_reg_vx(x, num & n);
                self.pc += 2;
            },
            0xD => {
                // draw(Vx,Vy,N)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.draw_sprite(bus, vx, vy, n)?;
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0x9E => {
                        // if(key()==Vx)
                        let key = self.read_reg_vx(x);
                        self.skip_if(bus.keyboard.is_key_pressed(key));
                    },
                    0xA1 => {
                        // 	if(key()!=Vx)
                        let key = self.read_reg_vx(x);
                        self.skip_if(!bus.keyboard.is_key_pressed(key));
                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    },
                }
            }
            0xF => {
                match nn {
                    0x7 => {
                        // Vx = get_delay()
                        self.write_reg_vx(x, bus.get_delay_timer());
                        self.pc += 2;
                    },
                    0x0A => {
                        // Vx = get_key()
                        self.pc += 2;
                    },
                    0x15 => {
                        // delay_timer(Vx)
                        bus.set_delay_timer(self.read_reg_vx(x));
                        self.pc += 2;
                    },
                    0x18 => {
                        // sound_timer(Vx)

                        // TODO
                        self.pc += 2;
                    },
                    0x1E => {
                        // I += Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    },
                    0x29 => {
                        // I=sprite_addr[Vx]
                        self.i = self.read_reg_vx(x) as u16 * 5;
                        self.pc += 2;
                    },
                    0x33 => {
                        // *(I+0)=BCD(3);
                        // *(I+1)=BCD(2);
                        // *(I+2)=BCD(1);
                        let vx = self.read_reg_vx(x);
                        bus.ram.write_byte(self.i as usize, vx / 100);
                        bus.ram.write_byte((self.i + 1) as usize, (vx % 100) / 10);
                        bus.ram.write_byte((self.i + 2) as usize, vx % 10);
                        self.pc += 2;
                    },
                    0x55 => {
                        // reg_dump(Vx,&I)
                    },
                    0x65 => {
                        // reg_load(Vx,&I)

                        for index in 0..x + 1 {
                            let value = bus.ram.read_byte((self.i + index as u16) as usize)?;
                            self.write_reg_vx(index, value);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;

                        //let index = (x+1) as usize;
                        //let bytes = bus.ram.read_bytes(self.i as usize, index)?;
                        //println!("Before: {:?}", self.vx);
                        //self.vx
                        //    .get_mut(0..index).ok_or("OOB index")?
                        //    .copy_from_slice(
                        //        bus.ram.read_bytes(self.i as usize, index)?
                        //    );
                        //println!("After: {:?}", self.vx);
                        //self.pc += 2;
                    },
                    _ => {
                        return Err(format!("Unrecognized opcode: {:#X}", instruction).into());
                    },
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

    /// Draws a sprite to the screen
    /// CHIP-8 sprites are always eight pixels wide and between one to fifteen pixels high
    /// One byte corresponds to one row of a given sprite
    pub fn draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, n: u8) -> Result<(), Box<dyn Error>> {
        log_special(
            format!(
                "Drawing a sprite at ({}, {}), of width {} and height {}",
                x,
                y,
                8,
                n,
            )
        );

        let mut should_set_vf = false;
        for sprite_y in 0..n {
            let byte = bus.ram.read_byte((self.i as u8 + sprite_y) as usize)?;
            if bus.display.draw_byte(byte, x, y + sprite_y)? {
                should_set_vf = true;
            }
        }

        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        }
        else {
            self.write_reg_vx(0xF, 0);
        }

        Ok(())
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpu {{\n")?;
        write!(f, "\tvx = {:X?}\n", self.vx)?;
        write!(f, "\tpc = {:X?}\n", self.pc)?;
        write!(f, "\ti = {:X?}\n", self.i)?;
        write!(f, "}}")
    }
}