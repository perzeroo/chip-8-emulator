use std::process;

pub struct Processor {
    pub registers: [u8; 16],
    pub address_register: u16,
    pub program_counter : usize,
    pub stack: [u16; 12],
    pub stack_pointer: usize,
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
            false
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
        }
    }
}
