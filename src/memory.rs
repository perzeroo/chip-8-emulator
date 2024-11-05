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
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let start_address = 0x200; // CHIP-8 occupied the first 512 bytes, on modern systems that is
                                  // no longer the case so we can store data there now such as font data.
        let end_address = start_address + buffer.len();
        self.data[start_address..end_address].copy_from_slice(&buffer);
        Ok(())
    }

    pub fn read_data(&self, address: usize) -> u8 {
        self.data[address]
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
