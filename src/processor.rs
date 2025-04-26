use std::fmt::Debug;

use crate::{addressing::AddressingMode, flags::Flags, instructions::Instruction, memory::Memory};
#[derive(Debug)]
pub struct Processor {
    memory: Memory,
    // Registers
    a: u8, // Accumulator
    x: u8,
    y: u8,
    p: Flags, // Processor Status
    // Adress in stack is 0x0100 + SP
    s: u8,   // Stack pointer
    pc: u16, // Program counter
}

impl Processor {
    /// Initialises a new `Processor` in its RESET state
    pub fn new(memory: Memory) -> Processor {
        Processor {
            memory,
            a: 0,
            x: 0,
            y: 0,
            p: Flags::new(),
            s: 0x00FD,
            // A bit after the zero page and stack
            pc: 0x1000,
        }
    }
    pub fn peek_byte_at_pc(&self) -> u8 {
        self.memory.read_byte(self.pc)
    }
    pub fn fetch_address(&mut self, addressing_mode: AddressingMode) -> u16 {
        self.pc += 1;
        match addressing_mode {
            AddressingMode::Absolute => {
                // Read two bytes
                let byte1 = self.peek_byte_at_pc() as u16;
                self.pc += 1;
                let byte2 = self.peek_byte_at_pc() as u16;

                let address = byte1 + (byte2 << 8);
                address
                // self.memory.read_byte(address)
            }
            AddressingMode::Immediate => {
                // PC is already at byte immediate mode needs
                self.pc
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn process_next_instruction(&mut self) {
        let value = self.memory.read_byte(self.pc);
        let instruction = Instruction::from(value);
        let addressing_mode = AddressingMode::from(value);
        dbg!(&instruction, &addressing_mode);
        let addr = self.fetch_address(addressing_mode);
        match instruction {
            // Load instructions
            Instruction::LDA => {
                // Load data into accumulator
                self.a = self.memory.read_byte(addr)
            }
            Instruction::LDX => self.x = self.memory.read_byte(addr),
            Instruction::LDY => self.y = self.memory.read_byte(addr),

            Instruction::STA => self.memory.write_byte(addr, self.a),

            _ => {}
        }
        self.pc += 1;
    }
}
