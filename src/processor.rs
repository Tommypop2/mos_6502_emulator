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
fn interpret_as_signed(v: u8) -> i8 {
    // Safe as we're converting a u8 into an i8, which is always valid
    unsafe { std::mem::transmute(v) }
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
    pub fn take_byte_at_pc(&mut self) -> u8 {
        let data = self.peek_byte_at_pc();
        self.pc += 1;
        data
    }
    /// Fetches the "destination" for the instruction.
    /// It is returned as a mutable reference to where the data is located (not yet but planned)
    pub fn fetch_address(&mut self, addressing_mode: AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::Immediate => {
                // PC is already at byte immediate mode needs
                self.pc
            }
            AddressingMode::ZeroPage => self.peek_byte_at_pc() as u16,
            AddressingMode::ZeroPageX => self.peek_byte_at_pc() as u16 + self.x as u16,
            AddressingMode::ZeroPageY => self.peek_byte_at_pc() as u16 + self.y as u16,
            AddressingMode::Absolute => {
                // Read two bytes
                let byte1 = self.peek_byte_at_pc() as u16;
                self.pc += 1;
                let byte2 = self.peek_byte_at_pc() as u16;

                let address = byte1 + (byte2 << 8);
                address
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn process_next_instruction(&mut self) {
        let value = self.take_byte_at_pc();
        let instruction = Instruction::from(value);
        let addressing_mode = AddressingMode::from(value);
        dbg!(&instruction, &addressing_mode);
        let addr = self.fetch_address(addressing_mode);
        // Not sure if incrementing pc before processing the instruction can cause issues.
        // It's mostly so the JMP instruction can set the PC properly
        self.pc += 1;
        match instruction {
            // Load instructions
            Instruction::LDA => {
                // Load data into accumulator
                self.a = self.memory.read_byte(addr)
            }
            Instruction::LDX => self.x = self.memory.read_byte(addr),
            Instruction::LDY => self.y = self.memory.read_byte(addr),

            Instruction::STA => self.memory.write_byte(addr, self.a),
            // Jump
            Instruction::JMP => self.pc = addr,
            Instruction::BCC => {
                if !self.p.get_carry_flag() {
                    self.pc += (2i8 + interpret_as_signed(self.memory.read_byte(addr))) as u16
                }
            }
            Instruction::BCS => {
                if self.p.get_carry_flag() {
                    self.pc += (2i8 + interpret_as_signed(self.memory.read_byte(addr))) as u16
                }
            }
            Instruction::BEQ => {
                if self.p.get_zero_flag() {
                    self.pc += (2i8 + interpret_as_signed(self.memory.read_byte(addr))) as u16
                }
            }
            Instruction::BMI => {
                if self.p.get_negative_flag() {
                    self.pc += (2i8 + interpret_as_signed(self.memory.read_byte(addr))) as u16
                }
            }
            Instruction::BNE => {
                if !self.p.get_zero_flag() {
                    self.pc += (2i8 + interpret_as_signed(self.memory.read_byte(addr))) as u16
                }
            }
            // Arithmetic
            Instruction::ADC => {
                let data = self.memory.read_byte(addr);
                let bit7_initial = (data & 0b10000000) != 0;
                let (res, overflowed) = self.a.overflowing_add(data);
                let bit7_result = (res & 0b10000000) != 0;
                // If the result and initial seventh bits aren't the same, then a signed overflow has occured
                if bit7_initial != bit7_result {
                    self.p.set_overflow_flag();
                } else {
                    self.p.clear_overflow_flag();
                }
                if overflowed {
                    self.p.set_carry_flag();
                } else {
                    self.p.clear_carry_flag();
                }
                // Negative value is this is true
                if bit7_result {
                    self.p.set_negative_flag();
                } else {
                    self.p.clear_negative_flag();
                }
                self.a = res;
            }

            Instruction::ASL => {
                let data = self.memory.read_byte(addr);
                // Get bit 7
                let bit7 = (data & 0b10000000) != 0;
                if bit7 {
                    self.p.set_carry_flag();
                } else {
                    self.p.clear_carry_flag();
                }
                // Write the result, but with bit 0 set to 0
                let result = (data << 1) & 0b11111110;
                // Get bit 7 of the result
                let bit7 = (data & 0b10000000) != 0;
                if bit7 {
                    self.p.set_negative_flag();
                } else {
                    self.p.clear_negative_flag();
                }
                self.memory.write_byte(addr, result);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loading_and_storing() {
        let bin = include_bytes!("../tests/loading_and_storing/test.bin");
        let mut memory = Memory::new();
        memory.write_bytes(0x1000, bin);
        let mut processor = Processor::new(memory);
        // Using 0 byte for program termination for now (which corresponds to the BRK instruction)
        while processor.peek_byte_at_pc() != 0 {
            processor.process_next_instruction();
        }

        assert_eq!(
            processor.memory.read_byte(0x1000),
            processor.memory.read_byte(0x100D)
        );
    }

    #[test]
    fn unsigned_addition() {
        let bin = include_bytes!("../tests/unsigned_addition/test.bin");
        let mut memory = Memory::new();
        memory.write_bytes(0x1000, bin);
        let mut processor = Processor::new(memory);
        // Using 0 byte for program termination for now (which corresponds to the BRK instruction)
        while processor.peek_byte_at_pc() != 0 {
            processor.process_next_instruction();
        }

        println!("{:#X?}", processor);
        assert_eq!(
            processor.memory.read_byte(0x1015) + processor.memory.read_byte(0x1016),
            processor.memory.read_byte(0x1017)
        )
    }
}
