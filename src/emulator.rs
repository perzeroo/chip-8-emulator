use crate::processor::*;
use crate::memory::*;
use crate::renderer::*;
use std::process;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Emulator {
    proc: Processor,
    pub mem: Memory,
    rng: ThreadRng,
    pub renderer: Renderer,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            proc: Processor::default(),
            mem: Memory::new(),
            rng: rand::thread_rng(),
            renderer: Renderer::new(),
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

        self.proc.address_register = 0x200;
        opcode = 0xD002;

        match opcode {
            0x00E0 => { // Clear the screen
                self.renderer.clear_pixels();
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

            // Vx |= Vy
            _ if (opcode & 0xF00F) == 0x8001 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                self.proc.set_register(register_x, register_x_value | register_y_value);
            }
            
            // Vx &= Vy
            _ if (opcode & 0xF00F) == 0x8002 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                self.proc.set_register(register_x, register_x_value & register_y_value);
            }

            // Vx ^= Vy
            _ if (opcode & 0xF00F) == 0x8003 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                self.proc.set_register(register_x, register_x_value ^ register_y_value);
            }

            // Vx += Vy
            _ if (opcode & 0xF00F) == 0x8004 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                let (value, overflowed) = register_x_value.overflowing_add(register_y_value);
                self.proc.set_register(register_x, value);
                self.proc.set_register(0xF, if overflowed {1} else {0});
            }

            // Vx -= Vy
            _ if (opcode & 0xF00F) == 0x8005 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                let (value, overflowed) = register_x_value.overflowing_sub(register_y_value);
                self.proc.set_register(register_x, value);
                self.proc.set_register(0xF, if overflowed {1} else {0});
            }

            // Vx >>= 1
            _ if (opcode & 0xF00F) == 0x8006 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let value = register_x_value >> 1;
                self.proc.set_register(register_x, value);
                self.proc.set_register(0xF, register_x_value & 1);
            }

            // Vx = Vy - Vx
            _ if (opcode & 0xF00F) == 0x8007 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let register_y_value = self.proc.get_register(((opcode >> 4) ^ 0x0F) as u8);
                let (value, overflowed) = register_y_value.overflowing_sub(register_x_value);
                self.proc.set_register(register_x, value);
                self.proc.set_register(0xF, if overflowed {1} else {0});
            }

            // Vx <<= 1
            _ if (opcode & 0xF00F) == 0x8008 => {
                let register_x = ((opcode >> 8) ^ 0x0F) as u8;
                let register_x_value = self.proc.get_register(register_x);
                let value = register_x_value << 1;
                self.proc.set_register(register_x, value);
                self.proc.set_register(0xF, register_x_value & 128);
            }

            // if (Vx == Vy) skip instruction
            _ if (opcode & 0xF00F) == 0x9000 => {
                let register_x_value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                let register_y_value = self.proc.get_register(((opcode >> 4) & 0x0F) as u8);

                if register_x_value != register_y_value {
                    self.proc.program_counter += 2;
                }
            }

            // address_register points to address NNN
            _ if (opcode & 0xF000) == 0xA000 => {
                self.proc.address_register = ((opcode << 4) as u16) >> 4;
            }

            // jump to address NNN
            _ if (opcode & 0xF000) == 0xB000 => {
                self.proc.program_counter = (((opcode << 4) as u16) >> 4) as usize;
            }
            
            // generate random number to register Vx and perform an & operation on it
            _ if (opcode & 0xF000) == 0xC000 => {
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, self.rng.gen::<u8>() & (opcode & 0x00FF) as u8);
            }

            _ if (opcode & 0xF000) == 0xD000 => {
                let x = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                let y = self.proc.get_register(((opcode >> 4) & 0x0F) as u8);
                let height = (opcode & 0x0F) as u8;
                let sprite_begin = self.proc.address_register;
                let sprite_end = self.proc.address_register + height as u16;

                let mut sprite = Vec::<u8>::new();
                for i in sprite_begin..sprite_end {
                    sprite.push(self.mem.read_data(i as usize));
                } 
                
                self.proc.set_register(0x0F, if self.renderer.draw_sprite(x, y, sprite) {1} else {0});
            }

            _ if (opcode & 0xF0FF) == 0xE09E => {
                let register_key = self.proc.
            }

            _default => {
                println!("Potentially unknown opcode? {:04X}", opcode);
            }
        }
    }
}
