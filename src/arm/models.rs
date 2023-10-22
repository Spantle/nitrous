use bitflags::bitflags;

// P30
enum Exception {
    Reset,
    UndefinedInstruction,
    SoftwareInterrupt, // SWI
    PrefetchAbort,
    DataAbort,
    Irq,
    Riq,
}

#[derive(Debug)]
pub struct PSR(u32);

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

impl PSR {
    const THUMB_OFFSET: u32 = 5;
    const FIQ_INTERRUPT_OFFSET: u32 = 6;
    const IRQ_INTERRUPT_OFFSET: u32 = 7;
    const STICKY_OFFSET: u32 = 27;
    const OVERFLOW_OFFSET: u32 = 28;
    const CARRY_OFFSET: u32 = 29;
    const ZERO_OFFSET: u32 = 30;
    const NEGATIVE_OFFSET: u32 = 31;

    pub fn default() -> PSR {
        PSR(ProcessorMode::SVC.bits())
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn from(value: u32) -> PSR {
        PSR(value)
    }

    fn get_bit(&self, offset: u32) -> bool {
        (self.0 >> offset) & 1 == 1
    }

    fn set_bit(&mut self, offset: u32, value: bool) {
        self.0 = (self.0 & !(1 << offset)) | ((value as u32) << offset);
    }

    pub fn mode(&self) -> ProcessorMode {
        ProcessorMode::from_bits_truncate(self.0 & ProcessorMode::MASK.bits())
    }

    pub fn set_mode(&mut self, mode: ProcessorMode) {
        self.0 = (self.0 & !ProcessorMode::MASK.bits()) | mode.bits();
    }

    pub fn thumb(&self) -> bool {
        self.get_bit(Self::THUMB_OFFSET)
    }

    pub fn set_thumb(&mut self, thumb: bool) {
        self.set_bit(Self::THUMB_OFFSET, thumb);
    }

    pub fn fiq_interrupt(&self) -> bool {
        self.get_bit(Self::FIQ_INTERRUPT_OFFSET)
    }

    pub fn set_fiq_interrupt(&mut self, fiq_interrupt: bool) {
        self.set_bit(Self::FIQ_INTERRUPT_OFFSET, fiq_interrupt)
    }

    pub fn irq_interrupt(&self) -> bool {
        self.get_bit(Self::IRQ_INTERRUPT_OFFSET)
    }

    pub fn set_irq_interrupt(&mut self, irq_interrupt: bool) {
        self.set_bit(Self::IRQ_INTERRUPT_OFFSET, irq_interrupt)
    }

    pub fn sticky(&self) -> bool {
        self.get_bit(Self::STICKY_OFFSET)
    }

    pub fn set_sticky(&mut self, sticky: bool) {
        self.set_bit(Self::STICKY_OFFSET, sticky)
    }

    pub fn overflow(&self) -> bool {
        self.get_bit(Self::OVERFLOW_OFFSET)
    }

    pub fn set_overflow(&mut self, overflow: bool) {
        self.set_bit(Self::OVERFLOW_OFFSET, overflow)
    }

    pub fn carry(&self) -> bool {
        self.get_bit(Self::CARRY_OFFSET)
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.set_bit(Self::CARRY_OFFSET, carry)
    }

    pub fn zero(&self) -> bool {
        self.get_bit(Self::ZERO_OFFSET)
    }

    pub fn set_zero(&mut self, zero: bool) {
        self.set_bit(Self::ZERO_OFFSET, zero)
    }

    pub fn negative(&self) -> bool {
        self.get_bit(Self::NEGATIVE_OFFSET)
    }

    pub fn set_negative(&mut self, negative: bool) {
        self.set_bit(Self::NEGATIVE_OFFSET, negative)
    }
}

// P40
struct Byte(i8);
struct HalfWord(i16);
struct Word(i32);
struct UByte(u8);
struct UHalfWord(u16);
struct UWord(u32);
