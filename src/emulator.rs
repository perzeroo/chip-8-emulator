use crate::processor::*;
use crate::memory::*;
use crate::renderer::*;
use crate::keyboard::*;
use std::mem;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use std::usize;
use rand::random;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Emulator {
    proc: Processor,
    pub mem: Memory,
    //pub renderer: Renderer,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            proc: Processor::default(),
            mem: Memory::new(),
            //renderer: Renderer::new(),
        }
    }
    
    pub fn prepare(&mut self) {
        self.proc.program_counter = 0x200;
    }
    
    pub fn clock(&mut self, renderer_mutex: Arc<Mutex<Renderer>>, keyboard_mutex: Arc<Mutex<Keyboard>>) {
        if self.proc.program_counter >= 4094 {
            eprintln!("Program counter exceeded 4095, max memory");
            process::exit(1);
        }

        self.proc.delay_timer.clock();

        let opcode: u16 = self.mem.read_instruction(self.proc.program_counter);
        self.proc.program_counter += 2;

        match opcode {
            0x00E0 => { // Clear the screen
                let mut renderer = renderer_mutex.lock().unwrap();
                (*renderer).clear_pixels();
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
                let value = register_value.overflowing_add(opcode as u8).0;
                self.proc.set_register(register_value, value);
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
                let random_num: u8 = random();
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, random_num & (opcode & 0x00FF) as u8);
            }

            // draw sprite to pixels array
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
                
                let mut renderer = renderer_mutex.lock().unwrap();
                self.proc.set_register(0x0F, if (*renderer).draw_sprite(x, y, sprite) {1} else {0});
            }

            // skips if key in Vx is pressed
            _ if (opcode & 0xF0FF) == 0xE09E => {
                let register_key = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                let mut keyboard = keyboard_mutex.lock().unwrap(); 
                if register_key == keyboard.get_hexkey_pressed() {
                    self.proc.program_counter += 2;
                }
            }

            // skips if key in Vx isnt pressed
            _ if (opcode & 0xF0FF) == 0xE0A1 => {
                let register_key = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                let mut keyboard = keyboard_mutex.lock().unwrap(); 
                if register_key != keyboard.get_hexkey_pressed() {
                    self.proc.program_counter += 2;
                }
            }

            // sets Vx to the value of the delay timer
            _ if (opcode & 0xF0FF) == 0xF007=> {
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, self.proc.delay_timer.value);
            }

            // sets the value of Vx to the pressed key, wait for key press
            _ if (opcode & 0xF0FF) == 0xF00A => {
                let mut keyboard = keyboard_mutex.lock().unwrap(); 
                let mut key = keyboard.get_hexkey_pressed();
                while key > 15 {
                    mem::drop(keyboard);
                    keyboard = keyboard_mutex.lock().unwrap();
                    key = keyboard.get_hexkey_pressed();
                }
                self.proc.set_register(((opcode >> 8) & 0x0F) as u8, keyboard.get_hexkey_pressed());
            }
            
            // sets the value of the delay timer to the value in Vx
            _ if (opcode & 0xF0FF) == 0xF015 => {
                self.proc.delay_timer.value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                println!("heere {}", self.proc.delay_timer.value);
            }

            // sets the value of the sound timer to the value of Vx
            _ if (opcode & 0xF0FF) == 0xF018 => {
            }

            // adds the value of Vx to I
            _ if (opcode & 0xF0FF) == 0xF01E => {
                self.proc.address_register += self.proc.get_register(((opcode >> 8) & 0x0F) as u8) as u16;
            }

            // sets I to the location of Vx font sprite
            _ if (opcode & 0xF0FF) == 0xF029 => {
                self.proc.address_register = self.proc.get_register(((opcode >> 8) & 0x0F) as u8) as u16 * 5;
            }

            // write value of Vx BCD coded at I
            _ if (opcode & 0xF0FF) == 0xF033 => {
                let register_x_value = self.proc.get_register(((opcode >> 8) & 0x0F) as u8);
                self.mem.write_data(self.proc.address_register as usize, register_x_value / 100);
                self.mem.write_data(self.proc.address_register as usize + 1, (register_x_value / 10) % 10);
                self.mem.write_data(self.proc.address_register as usize + 2, register_x_value % 10);
            }

            // write all registers up to Vx to memory at I
            _ if (opcode & 0xF0FF) == 0xF055 => {
                let end_index = ((opcode >> 8) & 0x0F) as u8;
                let register_data_array = self.proc.get_registers(end_index);

                for i in 0..(end_index+1) {
                    self.mem.write_data((self.proc.address_register + i as u16) as usize, register_data_array[i as usize]);
                }
            }

            _ if (opcode & 0xF0FF) == 0xF065 => {
                let end_index = ((opcode >> 8) & 0x0F) as u8;
                for i in 0..(end_index+1) {
                    self.proc.set_register(i, self.mem.read_data((self.proc.address_register + i as u16) as usize));
                }
            }

            _default => {
                println!("Potentially unknown opcode? {:04X}", opcode);
            }
        }
    }
}
