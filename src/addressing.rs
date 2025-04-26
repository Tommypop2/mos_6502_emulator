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

impl From<u8> for AddressingMode {
    fn from(opcode: u8) -> Self {
        let bbb = (opcode & 0b00011100) >> 2;
        let cc = opcode & 0b11;
        match cc {
            // Group One
            0b01 => match bbb {
                0b000 => Self::ZeroPageX,
                0b001 => Self::ZeroPage,
                0b010 => Self::Immediate,
                0b011 => Self::Absolute,
                0b100 => Self::ZeroPageY,
                0b101 => Self::ZeroPageX,
                0b110 => Self::AbsoluteY,
                0b111 => Self::AbsoluteX,
                _ => panic!(""),
            },
            // Group Two
            0b10 => match bbb {
                0b000 => Self::Immediate,
                0b001 => Self::ZeroPage,
                0b010 => Self::Accumulator,
                0b011 => Self::Absolute,
                0b101 => Self::ZeroPageX,
                0b111 => Self::AbsoluteX,
                _ => panic!(),
            },
            // Group Three
            0b00 => match bbb {
                0b000 => Self::Immediate,
                0b001 => Self::ZeroPage,
                0b011 => Self::Absolute,
                0b101 => Self::ZeroPageX,
                0b111 => Self::AbsoluteX,
                _ => panic!(),
            },
            _ => {
                panic!("")
            }
        }
    }
}
