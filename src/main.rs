use std::fs;

use memory::Memory;
use processor::Processor;

pub mod addressing;
pub mod flags;
pub mod instructions;
pub mod memory;
pub mod processor;
fn main() {
    let program_bytes = fs::read("./simple.bin").unwrap();
    let mut memory = Memory::new();
    memory.write_bytes(0x1000, &program_bytes);
    let mut processor = Processor::new(memory);
    // Using 0 byte for program termination for now (which corresponds to the BRK instruction)
    while processor.peek_byte_at_pc() != 0 {
        processor.process_next_instruction();
    }
		println!("{:#X?}", processor);
}
