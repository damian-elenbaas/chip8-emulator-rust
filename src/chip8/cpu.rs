use rand::random;
use std::io::{stdin, stdout, Write};

use crate::chip8::memory;

use super::memory::Memory;
use super::display::Display;

pub struct CPU {
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>
}

impl CPU {
    pub fn new() -> CPU {
        let offset = 0x200;

        CPU {
            registers: [0; 16],
            index_register: 0,
            program_counter: offset,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new()
        }
    }

    pub fn execute_instruction(&mut self, memory: &mut Memory, display: &mut Display) {
        let hi: u16 = memory.read_from_address(self.program_counter) as u16;
        let lo: u16 = memory.read_from_address(self.program_counter + 1) as u16;
        // println!("hi: {:#X}, lo: {:#X}", hi, lo);
        let opcode: u16 = (hi << 8) | lo;

        // println!("Executing opcode: {:#X}", opcode);

        if hi == 0 && lo == 0 {
            panic!("End of program");
        }
        self.program_counter += 2;

        let nnn: u16 = opcode & 0x0FFF;
        let nn: u8 = (opcode & 0x00FF) as u8;
        let n: u8 = (opcode & 0x000F) as u8;
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;

        let instruction = (opcode & 0xF000) >> 12 as u8;

        match instruction {
            0x0 => {
                match opcode {
                    0x00E0 => {
                        // Clear the screen
                        display.clear();
                    }

                    0x00EE => {
                        // Return from subroutine
                        self.program_counter = self.stack.pop().unwrap();
                    }

                    _ => {
                        panic!("Unknown opcode: {:X}", opcode);
                    }
                }
            }

            0x1 => {
                self.program_counter = nnn;
            }

            0x2 => {
                // Call subroutine at nnn
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
            }

            0x3 => {
                // Skip next instruction if Vx == nn
                let vx = self.registers[x as usize];
                if vx == nn {
                    self.program_counter += 2;
                }
            }

            0x4 => {
                // Skip next instruction if Vx != nn
                let vx = self.registers[x as usize];
                if vx != nn {
                    self.program_counter += 2;
                }
            }

            0x5 => {
                // Skip next instruction if Vx == Vy
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                if vx == vy {
                    self.program_counter += 2;
                }
            }

            0x6 => {
                // Set Vx = nn
                self.registers[x as usize] = nn;
            }

            0x7 => {
                // Set Vx = Vx + nn
                let vx = self.registers[x as usize];
                self.registers[x as usize] = vx.wrapping_add(nn);
            }

            0x8 => {
                match n {
                    0x0 => {
                        // Set Vx = Vy
                        let vy = self.registers[y as usize];
                        self.registers[x as usize] = vy;
                    }

                    0x1 => {
                        // Set Vx = Vx OR Vy
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        self.registers[x as usize] = vx | vy;
                    }

                    0x2 => {
                        // Set Vx = Vx AND Vy
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        self.registers[x as usize] = vx & vy;
                    }

                    0x3 => {
                        // Set Vx = Vx XOR Vy
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        self.registers[x as usize] = vx ^ vy;
                    }

                    0x4 => {
                        // Set Vx = Vx + Vy, set VF = carry
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        let (sum, overflow) = vx.overflowing_add(vy);
                        self.registers[x as usize] = sum;
                        self.registers[0xF] = overflow as u8;
                    }

                    0x5 => {
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        let (diff, overflow) = vx.overflowing_sub(vy);
                        self.registers[x as usize] = diff;
                        self.registers[0xF] = !overflow as u8;
                    }

                    0x6 => {
                        // Set Vx = Vx SHR 1
                        let vx = self.registers[x as usize];
                        self.registers[0xF] = vx & 0x1;
                        self.registers[x as usize] = vx >> 1;
                    }

                    0x7 => {
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        let (diff, overflow) = vy.overflowing_sub(vx);
                        self.registers[x as usize] = diff;
                        self.registers[0xF] = !overflow as u8;
                    }

                    0xE => {
                        // Set Vx = Vx SHL 1
                        let vx = self.registers[x as usize];
                        self.registers[0xF] = vx >> 7;
                        self.registers[x as usize] = vx << 1;
                    }

                    _ => {
                        panic!("Unknown instruction: {:X}", opcode);
                    }
                }
            }

            0x9 => {
                // Skip next instruction if Vx != Vy
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                if vx != vy {
                    self.program_counter += 2;
                }
            }

            0xA => {
                // Set I = nnn
                self.index_register = nnn;
            }

            0xB => {
                // Jump to location nnn + V0
                self.program_counter = nnn + self.registers[0] as u16;
            }

            0xC => {
                // Set Vx = random byte AND kk
                let random_byte = rand::random::<u8>();
                self.registers[x as usize] = random_byte & nn;
            }

            0xD => {
                // Draw a sprite
                let flipped = display.draw_sprite(x as usize, y as usize, n);
                if flipped {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }

            0xE => {
                match nn {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                        todo!();
                    }

                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                        todo!();
                    }

                    _ => {
                        panic!("Unknown instruction: {:X}", opcode);
                    }
                }
            }

            0xF => {
                match nn {
                    0x07 => {
                        // Set Vx = delay timer value
                        self.registers[x as usize] = self.delay_timer;
                    }

                    0x0A => {
                        // Wait for a key press, store the value of the key in Vx
                        todo!();
                    }

                    0x15 => {
                        // Set delay timer = Vx
                        self.delay_timer = self.registers[x as usize];
                    }

                    0x18 => {
                        // Set sound timer = Vx
                        self.sound_timer = self.registers[x as usize];
                    }

                    0x1E => {
                        // Set I = I + Vx
                        self.index_register += self.registers[x as usize] as u16;
                    }

                    0x29 => {
                        // Set I = location of sprite for digit Vx
                        let sprite_address = (self.registers[x as usize] as u16) * 5;
                        self.index_register = sprite_address;
                    }

                    0x33 => {
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2
                        let vx = self.registers[x as usize];
                        let hundreds = vx / 100;
                        let tens = (vx / 10) % 10;
                        let ones = vx % 10;

                        memory.write_to_address(self.index_register, hundreds);
                        memory.write_to_address(self.index_register+1, tens);
                        memory.write_to_address(self.index_register+2, ones);
                    }

                    0x55 => {
                        // Store registers V0 through Vx in memory starting at location I
                        for (i, register) in self.registers.iter().enumerate() {
                            memory.write_to_address(
                                self.index_register + i as u16, 
                                *register
                            );
                        }

                    }

                    0x65 => {
                        // Read registers V0 through Vx from memory starting at location I
                        for (i, register) in self.registers.iter_mut().enumerate() {
                            *register = memory.read_from_address(self.index_register + i as u16);
                        }
                    }

                    _ => {
                        panic!("Unknown instruction: {:X}", opcode);
                    }
                }
            }

            _ => {
                panic!("Unknown instruction: {:X}", opcode);
            }
        }
    }
}
