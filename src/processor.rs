use std::fmt::Debug;

use crate::{
    addressing::AddressingMode,
    flags::Flags,
    instructions::{
        ConditionalBranchInstruction, Group1Instruction, Group2Instruction, Group3Instruction,
        Instruction, SingleByteInstruction, SpecialCase,
    },
    memory::Memory,
};
#[derive(Debug)]
pub struct Processor {
    pub memory: Memory,
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
            s: 0x00FF,
            // A bit after the zero page and stack
            pc: 0x1000,
        }
    }
    pub fn add_to_pc(&mut self, num: i8) {
        let abs = num.unsigned_abs();
        if num < 0 {
            self.pc -= abs as u16
        } else {
            self.pc += abs as u16
        }
    }
    pub fn push_to_stack(&mut self, byte: u8) {
        self.memory.write_byte(self.s as u16, byte);
        self.s = self.s.wrapping_sub(1);
    }
    pub fn pop_from_stack(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.s as u16);
        self.s = self.s.wrapping_add(1);
        byte
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
    pub fn fetch_address(&mut self, addressing_mode: Option<AddressingMode>) -> u16 {
        let addressing_mode = if let Some(a) = addressing_mode {
            a
        } else {
            return u16::MAX;
        };
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

                byte1 + (byte2 << 8)
            }
            AddressingMode::Relative => {
                let offset = self.take_byte_at_pc() as i8 as u16;
                self.pc.wrapping_add(offset)
            }
            AddressingMode::Implicit => {
                // Will panic if this access is attempted
                u16::MAX
            }
            AddressingMode::Accumulator => u16::MAX,
            _ => {
                unimplemented!("Addressing mode {:?} isn't implemented", addressing_mode)
            }
        }
    }
    pub fn process_next_instruction(&mut self) {
        let value = self.take_byte_at_pc();
        let instruction = Instruction::from(value);
        let addressing_mode = AddressingMode::try_from((value, instruction)).ok();
        dbg!(&instruction, &addressing_mode);
        let addr = self.fetch_address(addressing_mode);

        match instruction {
            Instruction::GroupOne(instruction) => {
                match instruction {
                    Group1Instruction::ORA => self.a |= self.memory.read_byte(addr),
                    Group1Instruction::AND => self.a &= self.memory.read_byte(addr),
                    Group1Instruction::EOR => self.a ^= self.memory.read_byte(addr),
                    Group1Instruction::ADC => {
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
                    Group1Instruction::STA => self.memory.write_byte(addr, self.a),
                    Group1Instruction::LDA => {
                        // Load data into accumulator
                        self.a = self.memory.read_byte(addr)
                    }
                    Group1Instruction::CMP => {
                        let res = self.a.wrapping_sub(self.memory.read_byte(addr));
                        let is_negative = (res & 0b10000000) != 0;
                        if is_negative {
                            self.p.set_negative_flag();
                        } else {
                            self.p.clear_negative_flag();
                        }
                        if res == 0 {
                            self.p.set_zero_flag();
                        } else {
                            self.p.clear_zero_flag();
                        }
                        if !is_negative && res != 0 {
                            self.p.set_carry_flag();
                        } else {
                            self.p.clear_carry_flag();
                        }
                    }
                    Group1Instruction::SBC => todo!(),
                }
                self.pc += 1;
            }
            Instruction::GroupTwo(instruction) => {
                match instruction {
                    Group2Instruction::ASL => {
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
                    Group2Instruction::ROL => todo!(),
                    Group2Instruction::LSR => todo!(),
                    Group2Instruction::ROR => todo!(),
                    Group2Instruction::STX => todo!(),
                    Group2Instruction::LDX => self.x = self.memory.read_byte(addr),
                    Group2Instruction::DEC => todo!(),
                    Group2Instruction::INC => todo!(),
                }
                self.pc += 1;
            }
            Instruction::GroupThree(instruction) => {
                match instruction {
                    Group3Instruction::BIT => todo!(),
                    Group3Instruction::JMP => self.pc = addr,
                    Group3Instruction::JMPABS => todo!(),
                    Group3Instruction::STY => todo!(),
                    Group3Instruction::LDY => self.y = self.memory.read_byte(addr),
                    Group3Instruction::CPY => todo!(),
                    Group3Instruction::CPX => {
                        let res = self.x.wrapping_sub(self.memory.read_byte(addr));
                        let is_negative = (res & 0b10000000) != 0;
                        if is_negative {
                            self.p.set_negative_flag();
                        } else {
                            self.p.clear_negative_flag();
                        }
                        if res == 0 {
                            self.p.set_zero_flag();
                        } else {
                            self.p.clear_zero_flag();
                        }
                        if !is_negative && res != 0 {
                            self.p.set_carry_flag();
                        } else {
                            self.p.clear_carry_flag();
                        }
                    }
                }
                self.pc += 1;
            }
            Instruction::ConditionalBranch(instruction) => {
                match instruction {
                    ConditionalBranchInstruction::BPL => todo!(),
                    ConditionalBranchInstruction::BMI => {
                        if self.p.get_negative_flag() {
                            self.add_to_pc(interpret_as_signed(self.memory.read_byte(addr)));
                        }
                    }
                    ConditionalBranchInstruction::BVC => todo!(),
                    ConditionalBranchInstruction::BVS => todo!(),
                    ConditionalBranchInstruction::BCC => {
                        if !self.p.get_carry_flag() {
                            self.add_to_pc(interpret_as_signed(self.memory.read_byte(addr)));
                        }
                    }
                    ConditionalBranchInstruction::BCS => {
                        if self.p.get_carry_flag() {
                            self.add_to_pc(interpret_as_signed(self.memory.read_byte(addr)));
                        }
                    }
                    ConditionalBranchInstruction::BNE => {
                        if !self.p.get_zero_flag() {
                            dbg!("Branched!");
                            self.pc = addr;
                            // self.add_to_pc(interpret_as_signed(self.memory.read_byte(addr)));
                        }
                    }
                    ConditionalBranchInstruction::BEQ => {
                        if self.p.get_zero_flag() {
                            self.add_to_pc(interpret_as_signed(self.memory.read_byte(addr)));
                        }
                    }
                }
                // self.pc += 2;
                println!("0x{:X}", &self.pc);
            }
            Instruction::SingleByte(instruction) => match instruction {
                // Stack Operations
                // Push processor status onto stack
                SingleByteInstruction::PHP => self.push_to_stack(*self.p.raw()),
                // Pull processor status from stack
                SingleByteInstruction::PLP => *self.p.raw_mut() = self.pop_from_stack(),
                // Push accumulator onto stack
                SingleByteInstruction::PHA => self.push_to_stack(self.a),
                // Pull accumulator from stack
                SingleByteInstruction::PLA => self.a = self.pop_from_stack(),
                // Transfer X to stack pointer
                SingleByteInstruction::TXS => self.s = self.x,
                // Transfer stack pointer to X
                SingleByteInstruction::TSX => self.x = self.s,

                SingleByteInstruction::BRK => todo!(),
                SingleByteInstruction::RTI => todo!(),
                SingleByteInstruction::RTS => {
                    let byte1 = self.pop_from_stack();
                    let byte2 = self.pop_from_stack();
                    self.pc = ((byte1 as u16) << 8) + byte2 as u16 + 1
                }
                SingleByteInstruction::DEY => todo!(),
                SingleByteInstruction::TAY => todo!(),
                SingleByteInstruction::INY => todo!(),
                SingleByteInstruction::INX => todo!(),
                SingleByteInstruction::CLC => todo!(),
                SingleByteInstruction::SEC => todo!(),
                SingleByteInstruction::CLI => todo!(),
                SingleByteInstruction::SEI => todo!(),
                SingleByteInstruction::TYA => todo!(),
                SingleByteInstruction::CLV => todo!(),
                SingleByteInstruction::CLD => todo!(),
                SingleByteInstruction::SED => todo!(),
                SingleByteInstruction::TXA => todo!(),

                SingleByteInstruction::TAX => todo!(),

                SingleByteInstruction::DEX => {
                    self.x -= 1;
                    if self.x == 0 {
                        self.p.set_zero_flag();
                    } else {
                        self.p.clear_zero_flag();
                    }
                    if (self.x & 0b10000000) != 0 {
                        self.p.set_negative_flag();
                    } else {
                        self.p.clear_negative_flag();
                    }
                }
                SingleByteInstruction::NOP => self.pc += 1,
            },
            Instruction::SpecialCase(instruction) => match instruction {
                SpecialCase::JSRABS => {
                    // PC is already address (minus one) of return, as PC has been incremented by parsing address that JSR is acting on
                    let [byte1, byte2] = self.pc.to_le_bytes();
                    self.push_to_stack(byte1);
                    self.push_to_stack(byte2);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loading_and_storing() {
        let bin = include_bytes!("../tests/fixtures/loading_and_storing/test.bin");
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
        let bin = include_bytes!("../tests/fixtures/unsigned_addition/test.bin");
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
