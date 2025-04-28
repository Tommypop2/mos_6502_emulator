use std::fs;

use emulator::{memory::Memory, processor::Processor};

fn main() {
    let program_bytes = fs::read("./test.bin").unwrap();
    let mut memory = Memory::new();
    memory.write_bytes(0x1000, &program_bytes);
    let mut processor = Processor::new(memory);
    // Using 0 byte for program termination for now (which corresponds to the BRK instruction)
    while processor.peek_byte_at_pc() != 0 {
        processor.process_next_instruction();
    }
    println!("{:#X?}", processor);
}
