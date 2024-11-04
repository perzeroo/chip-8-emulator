use crate::processor::*;
use crate::memory::*;
use std::process;

pub struct Emulator {
    pub proc: Processor,
    pub mem: Memory,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            proc: Processor::default(),
            mem: Memory::new(),
        }
    }
    
    pub fn prepare(&mut self) {
        self.proc.program_counter = 0x200;
    }
    
    pub fn clock(&mut self) {
        if self.proc.program_counter >= 4094 {
            eprintln!("Program counter exceeded 4095, max memory");
            process::exit(1);
        }

        let mut opcode: u16 = self.mem.read_instruction(self.proc.program_counter);
        println!("Fetched instruction at 0x{:04X}: {:04X}", self.proc.program_counter, opcode);
        self.proc.program_counter += 2;

        opcode = 0x3ABC; 

        match opcode {
            0x00E0 => { // Clear the screen

            }

            0x00EE => { // Return out of subroutine
                self.proc.program_counter = self.proc.pop_stack() as usize;
            }

            _ if (opcode & 0xF000) == 0x1000 => { // Jump to address
                let address = opcode & 0x0FFF;
                self.proc.program_counter = address as usize;
            }

            _ if (opcode & 0xF000) == 0x2000 => { // Calls subroutine at address
                let address = opcode & 0x0FFF;
                self.proc.push_stack(self.proc.program_counter as u16);
                self.proc.program_counter = address as usize;
            }

            _ if (opcode & 0xF000) == 0x3000 => { // Skip next instruction if the value in register
                                                  // equal a value
                let value = opcode as u8; // Since the value is in the second byte we can simply
                                          // truncate using as and get the value
                let register_value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                if register_value == value {
                    self.proc.program_counter += 2;
                }
            }

            _ if (opcode & 0xF000) == 0x4000 => { // Skip next instruction if the value in register
                                                  // doesn't equal a value
                let value = opcode as u8; // Since the value is in the second byte we can simply
                                          // truncate using as and get the value
                let register_value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                if register_value != value {
                    self.proc.program_counter += 2;
                }
            }

            _ if (opcode & 0xF00F) == 0x5000 => {
                let register_x_value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                let register_y_value = self.proc.get_register(((opcode >> 4) & 0x0F) as u8);
                
                if register_y_value == register_x_value {
                    self.proc.program_counter += 2;
                }
            }

            _ if (opcode & 0xF000) == 0x6000 => { // Set value in register to a value 
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, opcode as u8);
            }

            // Adds value to register
            _ if (opcode & 0xF000) == 0x7000 => { 
                let register_value = ((opcode >> 8) & 0x0F) as u8;
                self.proc.set_register(register_value, register_value + opcode as u8);
            }

            // Sets value in register x to value in register y
            _ if (opcode & 0xF00F) == 0x8000 => {
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, self.proc.get_register(((opcode >> 4) & 0x0F) as u8));
            }


            _default => {
                println!("Potentially unknown opcode? {:04X}", opcode);
            }
        }
    }
}
