#![allow(dead_code)]

use bitflags::bitflags;

use super::Bits;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct PSR(pub u32);

bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct ProcessorMode: u32 {
        const SYS  = 0b11111; // System
        const UND  = 0b11011; // Undefined
        const ABT  = 0b10111; // Abort
        const SVC  = 0b10011; // Supervisor
        const IRQ  = 0b10010; // IRQ
        const FIQ  = 0b10001; // FIQ
        const USR  = 0b10000; // User
        const MASK = 0b11111;
    }
}

impl Default for PSR {
    fn default() -> PSR {
        PSR(ProcessorMode::SYS.bits())
    }
}

impl From<u32> for PSR {
    fn from(val: u32) -> Self {
        PSR(val)
    }
}

impl PSR {
    const THUMB_OFFSET: u32 = 5; // T
    const FIQ_INTERRUPT_OFFSET: u32 = 6; // F
    const IRQ_INTERRUPT_OFFSET: u32 = 7; // I
    const SATURATION_OFFSET: u32 = 27; // Q
    const OVERFLOW_OFFSET: u32 = 28; // V
    const CARRY_OFFSET: u32 = 29; // C
    const ZERO_OFFSET: u32 = 30; // Z
    const NEGATIVE_OFFSET: u32 = 31; // N

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn from(value: u32) -> PSR {
        PSR(value)
    }

    #[inline(always)]
    fn get_bit(&self, offset: u32) -> bool {
        self.0.get_bit(offset)
    }

    #[inline(always)]
    fn set_bit(&mut self, offset: u32, value: bool) {
        self.0.set_bit(offset, value)
    }

    #[inline(always)]
    pub fn set_bits(&mut self, offset: u32, end: u32, value: u32) {
        self.0.set_bits(offset, end, value)
    }

    pub fn get_mode(&self) -> ProcessorMode {
        ProcessorMode::from_bits_truncate(self.0 & ProcessorMode::MASK.bits())
    }

    pub fn set_mode(&mut self, mode: ProcessorMode) {
        self.0 = (self.0 & !ProcessorMode::MASK.bits()) | mode.bits();
    }

    pub fn get_thumb(&self) -> bool {
        self.get_bit(Self::THUMB_OFFSET)
    }

    pub fn set_thumb(&mut self, thumb: bool) {
        self.set_bit(Self::THUMB_OFFSET, thumb);
    }

    pub fn get_fiq_interrupt(&self) -> bool {
        self.get_bit(Self::FIQ_INTERRUPT_OFFSET)
    }

    pub fn set_fiq_interrupt(&mut self, fiq_interrupt: bool) {
        self.set_bit(Self::FIQ_INTERRUPT_OFFSET, fiq_interrupt)
    }

    pub fn get_irq_interrupt(&self) -> bool {
        self.get_bit(Self::IRQ_INTERRUPT_OFFSET)
    }

    pub fn set_irq_interrupt(&mut self, irq_interrupt: bool) {
        self.set_bit(Self::IRQ_INTERRUPT_OFFSET, irq_interrupt)
    }

    pub fn get_saturation(&self) -> bool {
        self.get_bit(Self::SATURATION_OFFSET)
    }

    pub fn set_saturation(&mut self, saturation: bool) {
        self.set_bit(Self::SATURATION_OFFSET, saturation)
    }

    pub fn get_overflow(&self) -> bool {
        self.get_bit(Self::OVERFLOW_OFFSET)
    }

    pub fn set_overflow(&mut self, overflow: bool) {
        self.set_bit(Self::OVERFLOW_OFFSET, overflow)
    }

    pub fn get_carry(&self) -> bool {
        self.get_bit(Self::CARRY_OFFSET)
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.set_bit(Self::CARRY_OFFSET, carry)
    }

    pub fn get_zero(&self) -> bool {
        self.get_bit(Self::ZERO_OFFSET)
    }

    pub fn set_zero(&mut self, zero: bool) {
        self.set_bit(Self::ZERO_OFFSET, zero)
    }

    pub fn get_negative(&self) -> bool {
        self.get_bit(Self::NEGATIVE_OFFSET)
    }

    pub fn set_negative(&mut self, negative: bool) {
        self.set_bit(Self::NEGATIVE_OFFSET, negative)
    }

    pub fn in_a_privileged_mode(&self) -> bool {
        self.get_mode() != ProcessorMode::USR
    }

    pub fn current_mode_has_spsr(&self) -> bool {
        self.get_mode() != ProcessorMode::USR && self.get_mode() != ProcessorMode::SYS
    }
}
