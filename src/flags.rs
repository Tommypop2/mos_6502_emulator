use std::fmt::Debug;

/// Wrapper around a `u8` with convenience methods for manually setting specific bits
impl Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0b{:b}", self.0)?;
        Ok(())
    }
}
pub struct Flags(u8);

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl Flags {
    pub fn new() -> Self {
        // This bit is initialised to 1 apparently: https://www.nesdev.org/wiki/Status_flags
        Self(0b00100000)
    }
    // Carry Flag
    pub fn get_carry_flag(&self) -> bool {
        (self.0 & 0b00000001) != 0
    }
    pub fn set_carry_flag(&mut self) {
        self.0 |= 0b00000001
    }
    pub fn clear_carry_flag(&mut self) {
        self.0 &= 0b11111110
    }
    // Zero Flag
    pub fn get_zero_flag(&self) -> bool {
        (self.0 & 0b00000010) != 0
    }
    pub fn set_zero_flag(&mut self) {
        self.0 |= 0b00000010
    }
    pub fn clear_zero_flag(&mut self) {
        self.0 &= 0b11111101
    }
    // Interrupt Disable Flag
    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.0 & 0b00000100) != 0
    }
    pub fn set_interrupt_disable_flag(&mut self) {
        self.0 |= 0b00000100
    }
    pub fn clear_interrupt_disable_flag(&mut self) {
        self.0 &= 0b11111011
    }
    // Decimal Mode Flag
    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.0 & 0b00001000) != 0
    }
    pub fn set_decimal_mode_flag(&mut self) {
        self.0 |= 0b00001000
    }
    pub fn clear_decimal_mode_flag(&mut self) {
        self.0 &= 0b11110111
    }
    // Break Command Flag
    pub fn get_break_command_flag(&self) -> bool {
        (self.0 & 0b00010000) != 0
    }
    pub fn set_break_command_flag(&mut self) {
        self.0 |= 0b00010000
    }
    pub fn clear_break_command_flag(&mut self) {
        self.0 &= 0b11101111
    }
    // Overflow flag
    pub fn get_overflow_flag(&self) -> bool {
        (self.0 & 0b01000000) != 0
    }
    pub fn set_overflow_flag(&mut self) {
        self.0 |= 0b01000000
    }
    pub fn clear_overflow_flag(&mut self) {
        self.0 &= 0b10111111
    }
    // Negative Flag
    pub fn get_negative_flag(&self) -> bool {
        (self.0 & 0b10000000) != 0
    }
    pub fn set_negative_flag(&mut self) {
        self.0 |= 0b10000000
    }
    pub fn clear_negative_flag(&mut self) {
        self.0 &= 0b01111111
    }
    pub fn raw(&self) -> &u8 {
        &self.0
    }
    pub fn raw_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carry_flag() {
        let mut flags = Flags::new();
        flags.set_carry_flag();
        assert!(flags.get_carry_flag());
        assert_eq!(flags.0, 0b00100001);
        flags.clear_carry_flag();
        assert!(!flags.get_carry_flag());
        assert_eq!(flags.0, 0b00100000);
    }

    #[test]
    fn all_flags() {
        let mut flags = Flags::new();
        flags.set_carry_flag();
        flags.set_zero_flag();
        flags.set_interrupt_disable_flag();
        flags.set_decimal_mode_flag();
        flags.set_break_command_flag();
        flags.set_overflow_flag();
        flags.set_negative_flag();
        assert_eq!(flags.0, 255);
        flags.clear_carry_flag();
        flags.clear_zero_flag();
        flags.clear_interrupt_disable_flag();
        flags.clear_decimal_mode_flag();
        flags.clear_break_command_flag();
        flags.clear_overflow_flag();
        flags.clear_negative_flag();
        assert_eq!(flags.0, 0b00100000);
    }
}
