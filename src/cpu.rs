use crate::ram::Ram;
use crate::screen::Screen;
use crate::keypad::Keypad;
use crate::utils::{log_debug, log_warning, register_error, stack_pop_error};

use std::error::Error;
use std::time::Instant;

use rand::Rng;

/// Address pointing to the start of the program (executable instructions) in a CHIP-8 ROM
pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
/// Struct emulating the CPU
pub struct Cpu {
    /// Vector containing the 16 8-bit registers, referred as V0 to VF
    vx: Vec<u8>,
    /// Program Counter (or Instruction Pointer)
    pc: u16,
    /// The 16-bit register used to store memory addresses,
    i: u16,
    /// Vector emulating the stack containing 16 16-bit values at maximum
    /// Only used for return addresses in CHIP-8
    stack: Vec<u16>,
    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,
    debug_time: Instant,
    debug_count: u64,
}

impl Cpu {
    /// Creates a new `Cpu` object
    pub fn new() -> Self {
        Cpu {
            vx: vec![0; 16],
            pc: PROGRAM_START,
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            debug_time: Instant::now(),
            debug_count: 0,
        }
    }

    /// Runs a single instruction at `pc` address
    pub fn run_instruction(&mut self, ram: &mut Ram, screen: &mut Screen, keypad: &Keypad, debug: bool) -> Result<(), Box<dyn Error>> {
        // Big-endian address
        let high = ram.read_byte(self.pc as usize)? as u16;
        let low = ram.read_byte((self.pc + 1) as usize)? as u16;
        let instruction: u16 = (high << 8) | low;

        if debug {
            log_debug(
                format!(
                    "Instruction: {:#X?}, high: {:#X?}, low: {:#X?}", instruction, high, low
                )
            );
        }

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x00FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        if debug {
            log_debug(
                format!(
                    "nnn: {:X?}, nn: {:X?}, n: {:X?}, x: {:X?}, y: {:X?}", nnn, nn, n, x, y
                )
            );
        }

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                // I choose not to implement opcode 0x0NNN, only used on older machines and deprecated
                match nn {
                    0xE0 => {
                        // disp_clear()
                        screen.clear();
                        self.pc += 2;
                    },
                    0xEE => {
                        // return;
                        self.pc = self.stack.pop().ok_or(
                            stack_pop_error()
                        )?;
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
                // if (Vx == NN)
                let vx = self.read_reg_vx(x);
                self.skip_if(vx == nn);
            },
            0x4 => {
                // if (Vx != NN)
                let vx = self.read_reg_vx(x);
                self.skip_if(vx != nn);
            },
            0x5 => {
                // if (Vx == Vy)
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
                        // Vx = Vy
                        self.write_reg_vx(x, vy);
                    },
                    0x1 => {
                        // Vx = Vx | Vy
                        self.write_reg_vx(x, vx | vy);
                    },
                    0x2 => {
                        // Vx = Vx & Vy
                        self.write_reg_vx(x, vx & vy);
                    },
                    0x3 => {
                        // 	Vx = Vx ^ Vy
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    0x4 => {
                        // Vx += Vy
                        let sum = vx as u16 + vy as u16;
                        self.write_reg_vx(x, (sum % 256) as u8);
                        if sum > 0xFF {
                            if debug {
                                log_warning(
                                    format!(
                                        "Overflow detected on instruction: {:X}", instruction
                                    )
                                );
                            }

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
                        // Vx >>= 1
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    },
                    0x7 => {
                        // Vx = Vy - Vx
                        if vy > vx {
                            self.write_reg_vx(0xF, 1);
                        }
                        else {
                            self.write_reg_vx(0xF, 0);
                        }
                        self.write_reg_vx(x, vy - vx);
                    },
                    0xE => {
                        // Vx <<= 1
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
                // if (Vx != Vy)
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
                // PC = V0 + NNN
                self.pc = self.read_reg_vx(0) as u16 + nnn;
            },
            0xC => {
                // Vx = rand() & NN
                let mut rng = rand::thread_rng();
                let num: u8 = rng.gen_range(0, 255);
                self.write_reg_vx(x, num & n);
                self.pc += 2;
            },
            0xD => {
                // draw(Vx,Vy,N)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.draw_sprite(ram, screen, debug, vx, vy, n)?;
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0x9E => {
                        // if (key() == Vx)
                        let key = self.read_reg_vx(x);
                        self.skip_if(*keypad.is_key_pressed(key)?);
                    },
                    0xA1 => {
                        // 	if (key() != Vx)
                        let key = self.read_reg_vx(x);
                        self.skip_if(!(*keypad.is_key_pressed(key)?));
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
                        self.write_reg_vx(x, self.delay_timer);
                        self.pc += 2;
                    },
                    0x0A => {
                        // Vx = get_key()
                        if let Some(key) = keypad.first_pressed_key() {
                            self.write_reg_vx(x, key);
                            self.pc += 2;
                        }
                    },
                    0x15 => {
                        // delay_timer(Vx)
                        self.delay_timer = self.read_reg_vx(x);
                        self.pc += 2;
                    },
                    0x18 => {
                        // sound_timer(Vx)
                        self.sound_timer = self.read_reg_vx(x);
                        self.pc += 2;
                    },
                    0x1E => {
                        // I += Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    },
                    0x29 => {
                        // I = sprite_addr[Vx]
                        self.i = self.read_reg_vx(x) as u16 * 5;
                        self.pc += 2;
                    },
                    0x33 => {
                        // *(I+0) = BCD(3);
                        // *(I+1) = BCD(2);
                        // *(I+2) = BCD(1);
                        let vx = self.read_reg_vx(x);
                        ram.write_byte(self.i as usize, vx / 100)?;
                        ram.write_byte((self.i + 1) as usize, (vx % 100) / 10)?;
                        ram.write_byte((self.i + 2) as usize, vx % 10)?;
                        self.pc += 2;
                    },
                    0x55 => {
                        // reg_dump(Vx,&I)
                        // https://stackoverflow.com/questions/51179156/increment-i-in-chip-8-opcode-fx65
                        // Here, we use the old method from the 70s
                        // I += x+1

                        let index = (x+1) as usize;
                        ram.write_bytes(
                            self.i as usize,
                            self.vx.get(0..index).ok_or(
                                register_error(index)
                            )?
                        )?;
                        self.i += x as u16 + 1;

                        // TODO
                        self.pc += 2;

                    },
                    0x65 => {
                        // reg_load(Vx,&I)
                        // https://stackoverflow.com/questions/51179156/increment-i-in-chip-8-opcode-fx65
                        // Here, we use the old method from the 70s
                        // I += x+1

                        let index = (x+1) as usize;
                        self.vx
                            .get_mut(0..index).ok_or(
                                register_error(index)
                            )?
                            .copy_from_slice(
                                ram.read_bytes(self.i as usize, index)?
                            );
                        self.i += x as u16 + 1;

                        self.pc += 2;
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

    /// Writes in register at index `index` the value `value`
    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    /// Reads value from register at index `index`
    pub fn read_reg_vx(&self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    /// Skips the next instruction if the condition `cond` is true
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
    pub fn draw_sprite(&mut self, ram: &mut Ram, screen: &mut Screen, debug: bool, x: u8, y: u8, n: u8) -> Result<(), Box<dyn Error>> {
        if debug {
            log_debug(
                format!(
                    "Drawing a sprite at ({}, {}), of width {} and height {}",
                    x,
                    y,
                    8,
                    n,
                )
            );
        }

        let mut should_set_vf = false;
        for sprite_y in 0..n {
            let byte = ram.read_byte((self.i + sprite_y as u16) as usize)?;
            if screen.draw_byte(x, y + sprite_y, byte)? {
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

    /// Updates the timers (delay_timer / sound_timer)
    ///
    /// Returns whether the sound timer is active or not
    pub fn update_timers(&mut self, debug: bool) -> bool {
        if debug {
            self.debug_count += 1;
            if self.debug_count == 60 {
                // This should be 1 second
                let time_taken = Instant::now() - self.debug_time;
                self.debug_time = Instant::now();
                self.debug_count = 0;
                log_debug(
                    format!(
                        "Time taken for 60 update_timers: {:?}", time_taken
                    )
                );
            }
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        self.sound_timer > 0
    }
}
