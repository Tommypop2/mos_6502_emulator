use crate::instructions::{ConditionalBranchInstruction, Instruction};

#[derive(Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Indirect,

    // More annoying modes
    ZeroPageX,
    ZeroPageY,

    AbsoluteX,
    AbsoluteY,

    IndirectX,
    IndirectY,
}
type OpCodeInstructionPair = (u8, Instruction);

impl TryFrom<OpCodeInstructionPair> for AddressingMode {
    type Error = ();
    fn try_from((opcode, instruction): (u8, Instruction)) -> Result<Self, ()> {
        // Handle special cases (instructions which cannot be handled entirely by group)
        match instruction {
            Instruction::ConditionalBranch(_) => return Ok(Self::Relative),
						_ => {}
        }
        // Return an error if it's a single byte instruction
        let bbb = (opcode & 0b00011100) >> 2;
        let cc = opcode & 0b11;
        match cc {
            // Group One
            0b01 => match bbb {
                0b000 => Ok(Self::ZeroPageX),
                0b001 => Ok(Self::ZeroPage),
                0b010 => Ok(Self::Immediate),
                0b011 => Ok(Self::Absolute),
                0b100 => Ok(Self::ZeroPageY),
                0b101 => Ok(Self::ZeroPageX),
                0b110 => Ok(Self::AbsoluteY),
                0b111 => Ok(Self::AbsoluteX),
                _ => Err(()),
            },
            // Group Two
            0b10 => match bbb {
                0b000 => Ok(Self::Immediate),
                0b001 => Ok(Self::ZeroPage),
                0b010 => Ok(Self::Accumulator),
                0b011 => Ok(Self::Absolute),
                0b101 => Ok(Self::ZeroPageX),
                0b111 => Ok(Self::AbsoluteX),
                _ => Err(()),
            },
            // Group Three
            0b00 => match bbb {
                0b000 => Ok(Self::Immediate),
                0b001 => Ok(Self::ZeroPage),
                0b011 => Ok(Self::Absolute),
                0b101 => Ok(Self::ZeroPageX),
                0b111 => Ok(Self::AbsoluteX),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}
