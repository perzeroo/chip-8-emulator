use std::{process, time::{Duration, Instant}};

pub struct DelayTimer {
    pub value: u8,
    pub last_update: Instant,
    pub interval: Duration,
}

pub struct Processor {
    pub registers: [u8; 16],
    pub address_register: u16,
    pub program_counter : usize,
    pub stack: [u16; 12],
    pub stack_pointer: usize,
    pub delay_timer: DelayTimer,
}

impl DelayTimer {
    pub fn new() -> Self {
        DelayTimer {
            value: 0,
            last_update: Instant::now(),
            interval: Duration::from_secs_f64(1.0 / 60.0),
        }
    }

    pub fn clock(&mut self) {
        if self.last_update.elapsed() >= self.interval {
            if self.value > 0 {
                self.value -= 1;
            }
            self.last_update = Instant::now();
        }
    }
}

impl Processor {
    pub fn get_register(&self, index: u8) -> u8 {
        if index < 16 {
            self.registers[index as usize]
        } else {
            0
        }
    }

    pub fn set_register(&mut self, index: u8, value: u8) -> bool {
        if index < 16 {
            self.registers[index as usize] = value;
            true
        } else {
            println!("Tried to index register larger than 0xF");
            false
        }
    }

    pub fn get_registers(&self, end_index: u8) -> [u8; 16] {
        if end_index < 16 {
            let mut return_array: [u8; 16] = [0; 16];
            for i in 0..(end_index+1) {
                return_array[i as usize] = self.registers[i as usize];
            }
            return_array
        }  else {
            [0; 16]
        }
    }

    pub fn push_stack(&mut self, value: u16) {
        if self.stack_pointer < self.stack.len() {
            self.stack[self.stack_pointer] = value;
            self.stack_pointer += 1;
        } else {
            eprintln!("Stack overflow, exiting");
            process::exit(1);
        }
    }

    pub fn pop_stack(&mut self) -> u16 {
        if self.stack_pointer > 0 {
            self.stack_pointer -= 1;
            self.stack[self.stack_pointer]
        } else {
            eprintln!("Stack underflow, exiting");
            process::exit(1);
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Processor {
            registers: [0; 16],
            address_register: 0,
            program_counter: 0,
            stack: [0; 12],
            stack_pointer: 0,
            delay_timer: DelayTimer::new(),
        }
    }
}
