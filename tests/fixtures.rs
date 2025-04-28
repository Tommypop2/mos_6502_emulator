use std::{fs, path::PathBuf};

use emulator::{memory::Memory, processor::Processor};
use testing::fixture;

#[fixture("tests/fixtures/**/test.bin")]
fn fixture_tests(input: PathBuf) {
    let program = fs::read(&input).unwrap();
    let mut memory = Memory::new();
    memory.write_bytes(0x1000, &program);
    let mut processor = Processor::new(memory);
    // Using 0 byte for program termination for now (which corresponds to the BRK instruction)
    while processor.peek_byte_at_pc() != 0 {
        processor.process_next_instruction();
    }
    let start_region = 0x1000;
    let num_bytes_to_read = 1000;
    let expected_memory_path = input.parent().unwrap().join("expected_memory.bin");
    if !expected_memory_path.exists() {
        let bytes = processor.memory.read_bytes(start_region, num_bytes_to_read);
        fs::write(expected_memory_path, bytes).unwrap();
        return;
    }
    let expected_bytes = fs::read(expected_memory_path).unwrap();
    assert_eq!(
        expected_bytes,
        processor.memory.read_bytes(start_region, num_bytes_to_read)
    )
}
