use std::{fmt::Debug, io::Read};

const MEMORY_SIZE: usize = 0x10000;
impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Only show some around 0x1000 (where the test programs will be reading and writing memory)
        let region = self.read_bytes(0x1000, 256);
        let mut i = 0;
        for val in region {
            write!(f, "{:x}, ", val)?;
            i += 1;
            if i % 16 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Create new `Memory`, initialised to 0
    pub fn new() -> Self {
        Memory([0; MEMORY_SIZE])
    }
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.0[addr as usize] = byte;
    }
    pub fn write_bytes(&mut self, addr: u16, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.0[addr as usize + i] = *byte;
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }
    pub fn mut_byte(&mut self, addr: u16) -> &mut u8 {
        &mut self.0[addr as usize]
    }
    pub fn read_bytes(&self, addr: u16, number: u16) -> &[u8] {
        &self.0[(addr as usize)..((addr + number) as usize)]
    }
    pub fn to_bytes(&self) {
        self.0.bytes();
    }
}
