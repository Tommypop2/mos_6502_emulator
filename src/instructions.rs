#[derive(Debug)]
pub enum Instruction {
    // Group One
    ORA,
    AND,
    EOR,
    ADC,
    STA,
    LDA,
    CMP,
    SBC,

    // Group Two
    ASL,
    ROL,
    LSR,
    ROR,
    STX,
    LDX,
    DEC,
    INC,

    // Group Three
    BIT,
    JMP,
    JMPABS,
    STY,
    LDY,
    CPY,
    CPX,

    // Conditional Branches
    BPL,
    BMI,
    BVC,
    BVS,
    BCC,
    BCS,
    BNE,
    BEQ,

    // Other single byte instructions
    BRK,
    JSRABS,
    RTI,
    RTS,
}
// Parse instructions that don't follow aaabbbcc rule
fn parse_single_byte_instructions(opcode: u8) -> Option<Instruction> {
    if opcode & 0b11111 == 0b10000 {
        return Some(match opcode {
            0x10 => Instruction::BPL,
            0x30 => Instruction::BMI,
            0x50 => Instruction::BVC,
            0x70 => Instruction::BVS,
            0x90 => Instruction::BCC,
            0xB0 => Instruction::BCS,
            0xD0 => Instruction::BNE,
            0xF0 => Instruction::BEQ,
            _ => panic!("Unsupported!"),
        });
    }
    // Check for other single byte instructions
    match opcode {
        0x00 => Some(Instruction::BRK),
        0x20 => Some(Instruction::JSRABS),
        0x40 => Some(Instruction::RTI),
        0x60 => Some(Instruction::RTS),
        _ => None,
    }
}
impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        if let Some(instruction) = parse_single_byte_instructions(opcode) {
            return instruction;
        }
        let cc = opcode & 0b11;
        let aaa = (opcode & 0b11100000) >> 5;
        match cc {
            // Group One
            0b01 => match aaa {
                0b000 => Self::ORA,
                0b001 => Self::AND,
                0b010 => Self::EOR,
                0b011 => Self::ADC,
                0b100 => Self::STA,
                0b101 => Self::LDA,
                0b110 => Self::CMP,
                0b111 => Self::SBC,
                _ => panic!("Unsupported instruction"),
            },
            // Group Two
            0b10 => match aaa {
                0b000 => Self::ASL,
                0b001 => Self::LSR,
                0b010 => Self::ROR,
                0b100 => Self::STX,
                0b101 => Self::LDX,
                0b110 => Self::DEC,
                0b111 => Self::INC,
                _ => panic!("Unsupported"),
            },
            0b00 => match aaa {
                0b001 => Self::BIT,
                0b010 => Self::JMP,
                0b011 => Self::JMPABS,
                0b100 => Self::STY,
                0b101 => Self::LDY,
                0b110 => Self::CPY,
                0b111 => Self::CPX,
                _ => panic!("Unsupported"),
            },
            _ => panic!("Unsupported instruction, opcode ${:x}", opcode),
        }
    }
}
