// Instructions are grouped as shown on https://llx.com/Neil/a2/opcodes.html
#[derive(Clone, Copy, Debug)]
pub enum Group1Instruction {
    // Group One
    ORA,
    AND,
    EOR,
    ADC,
    STA,
    LDA,
    CMP,
    SBC,
}
#[derive(Clone, Copy, Debug)]
pub enum Group2Instruction {
    // Group Two
    ASL,
    ROL,
    LSR,
    ROR,
    STX,
    LDX,
    DEC,
    INC,
}
#[derive(Clone, Copy, Debug)]
pub enum Group3Instruction {
    // Group Three
    BIT,
    JMP,
    // JMPABS,
    STY,
    LDY,
    CPY,
    CPX,
}
#[derive(Clone, Copy, Debug)]
pub enum ConditionalBranchInstruction {
    // Conditional Branches
    BPL,
    BMI,
    BVC,
    BVS,
    BCC,
    BCS,
    BNE,
    BEQ,
}
#[derive(Clone, Copy, Debug)]
pub enum SingleByteInstruction {
    // Other single byte instructions
    // Interrupt and subroutine
    BRK,
    RTI,
    RTS,

    PHP,
    PLP,
    PHA,
    PLA,
    DEY,
    TAY,
    INY,
    INX,
    CLC,
    SEC,
    CLI,
    SEI,
    TYA,
    CLV,
    CLD,
    SED,
    TXA,
    TXS,
    TAX,
    TSX,
    DEX,
    NOP,
}
// Feels slightly overkill to have this separate group for a single instruction, but it doesn't fit the pattern anywhere else
#[derive(Clone, Copy, Debug)]
pub enum SpecialCase {
    // Apparently "only absolute-addressing instruction that doesn't fit the aaabbbcc"
    JSRABS,
}
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    GroupOne(Group1Instruction),
    GroupTwo(Group2Instruction),
    GroupThree(Group3Instruction),
    ConditionalBranch(ConditionalBranchInstruction),
    SingleByte(SingleByteInstruction),
    SpecialCase(SpecialCase),
}
// Parse instructions that don't follow aaabbbcc rule
fn parse_conditional_branch_instruction(opcode: u8) -> Option<ConditionalBranchInstruction> {
    if opcode & 0b11111 == 0b10000 {
        return Some(match opcode {
            0x10 => ConditionalBranchInstruction::BPL,
            0x30 => ConditionalBranchInstruction::BMI,
            0x50 => ConditionalBranchInstruction::BVC,
            0x70 => ConditionalBranchInstruction::BVS,
            0x90 => ConditionalBranchInstruction::BCC,
            0xB0 => ConditionalBranchInstruction::BCS,
            0xD0 => ConditionalBranchInstruction::BNE,
            0xF0 => ConditionalBranchInstruction::BEQ,
            _ => panic!("Unsupported!"),
        });
    }
    None
}
fn parse_special_case(opcode: u8) -> Option<SpecialCase> {
    if opcode == 0x20 {
        Some(SpecialCase::JSRABS)
    } else {
        None
    }
}
fn parse_other_single_byte_instruction(opcode: u8) -> Option<SingleByteInstruction> {
    match opcode {
        0x00 => Some(SingleByteInstruction::BRK),
        0x40 => Some(SingleByteInstruction::RTI),
        0x60 => Some(SingleByteInstruction::RTS),

        0x08 => Some(SingleByteInstruction::PHP),
        0x28 => Some(SingleByteInstruction::PLP),
        0x48 => Some(SingleByteInstruction::PHA),
        0x68 => Some(SingleByteInstruction::PLA),
        0x88 => Some(SingleByteInstruction::DEY),
        0xA8 => Some(SingleByteInstruction::TAY),
        0xC8 => Some(SingleByteInstruction::INY),
        0xE8 => Some(SingleByteInstruction::INX),

        0x18 => Some(SingleByteInstruction::CLC),
        0x38 => Some(SingleByteInstruction::SEC),
        0x58 => Some(SingleByteInstruction::CLI),
        0x78 => Some(SingleByteInstruction::SEI),
        0x98 => Some(SingleByteInstruction::TYA),
        0xB8 => Some(SingleByteInstruction::CLV),
        0xD8 => Some(SingleByteInstruction::CLD),
        0xF8 => Some(SingleByteInstruction::SED),

        0x8A => Some(SingleByteInstruction::TXA),
        0x9A => Some(SingleByteInstruction::TXS),
        0xAA => Some(SingleByteInstruction::TAX),
        0xBA => Some(SingleByteInstruction::TSX),
        0xCA => Some(SingleByteInstruction::DEX),
        0xEA => Some(SingleByteInstruction::NOP),
        _ => None,
    }
}
fn parse_single_byte_instruction(opcode: u8) -> Option<Instruction> {
    if let Some(conditional_branch) = parse_conditional_branch_instruction(opcode) {
        return Some(Instruction::ConditionalBranch(conditional_branch));
    }
    if let Some(special_case) = parse_special_case(opcode) {
        return Some(Instruction::SpecialCase(special_case));
    }
    // Check for other single byte instructions
    parse_other_single_byte_instruction(opcode).map(Instruction::SingleByte)
}
impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        if let Some(instruction) = parse_single_byte_instruction(opcode) {
            return instruction;
        }
        let cc = opcode & 0b11;
        let aaa = (opcode & 0b11100000) >> 5;
        match cc {
            // Group One
            0b01 => Instruction::GroupOne(match aaa {
                0b000 => Group1Instruction::ORA,
                0b001 => Group1Instruction::AND,
                0b010 => Group1Instruction::EOR,
                0b011 => Group1Instruction::ADC,
                0b100 => Group1Instruction::STA,
                0b101 => Group1Instruction::LDA,
                0b110 => Group1Instruction::CMP,
                0b111 => Group1Instruction::SBC,
                _ => panic!("Unsupported instruction"),
            }),
            // Group Two
            0b10 => Instruction::GroupTwo(match aaa {
                0b000 => Group2Instruction::ASL,
                0b001 => Group2Instruction::LSR,
                0b010 => Group2Instruction::ROR,
                0b100 => Group2Instruction::STX,
                0b101 => Group2Instruction::LDX,
                0b110 => Group2Instruction::DEC,
                0b111 => Group2Instruction::INC,
                _ => panic!("Unsupported"),
            }),
            0b00 => Instruction::GroupThree(match aaa {
                0b001 => Group3Instruction::BIT,
                0b010 => Group3Instruction::JMP,
                // 0b011 => Group3Instruction::JMPABS,
                0b100 => Group3Instruction::STY,
                0b101 => Group3Instruction::LDY,
                0b110 => Group3Instruction::CPY,
                0b111 => Group3Instruction::CPX,
                _ => panic!("Unsupported"),
            }),
            _ => panic!("Unsupported instruction, opcode ${:x}", opcode),
        }
    }
}
