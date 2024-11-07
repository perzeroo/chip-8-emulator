use std::fs::File;
use std::io::{self, Read};

pub struct Memory {
    data: [u8; 0x1000], // The computers running CHIP-8 had 4096 (0x1000) bytes of ram
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 0x1000] } // Initializes all bytes to 0
    }

    pub fn load_rom(&mut self, file_path: &str) -> io::Result<()> {
        self.load_rom_at_location(file_path, 0x200)
    }
    
    pub fn load_rom_at_location(&mut self, file_path: &str, start_address: usize) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let end_address = start_address + buffer.len();
        self.data[start_address..end_address].copy_from_slice(&buffer);
        Ok(())

    }

    pub fn read_data(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write_data(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }

    pub fn print_mem(&self) {
        for (i, val) in self.data.iter().enumerate() {
            print!("{:02X} ", val);
            if i > 0 && i % 128 == 0 {
                println!("");
            }
        }
    }
    
    pub fn read_instruction(&self, address: usize) -> u16 { // Instructions on the CHIP-8 are
                                                            // 16bits long, also it's in big endian
                                                            // format so we put the low byte after
                                                            // high byte
        let high = self.data[address];
        let low = self.data[address + 1];
        ((high as u16) << 8) | (low as u16)
    }
}
