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

bitflags! {
    #[derive(Eq, PartialEq)]
    pub struct PSR: u32 {
        const MODE_USR      = 0b00000000000000000000000000010000; // User       0b10000
        const MODE_FIQ      = 0b00000000000000000000000000010001; // FIQ        0b10001
        const MODE_IRQ      = 0b00000000000000000000000000010010; // IRQ        0b10010
        const MODE_SVC      = 0b00000000000000000000000000010011; // Supervisor 0b10011
        const MODE_ABT      = 0b00000000000000000000000000010111; // Abort      0b10111
        const MODE_UND      = 0b00000000000000000000000000011011; // Undefined  0b11011
        const MODE_SYS      = 0b00000000000000000000000000011111; // System     0b11111
        const MODE_MASK     = 0b00000000000000000000000000011111;
        const THUMB         = 0b00000000000000000000000000100000;
        const FIQ_INTERRUPT = 0b00000000000000000000000001000000;
        const IRQ_INTERRUPT = 0b00000000000000000000000010000000;
        const STICKY        = 0b00001000000000000000000000000000;
        const OVERFLOW      = 0b00010000000000000000000000000000;
        const CARRY         = 0b00100000000000000000000000000000;
        const ZERO          = 0b01000000000000000000000000000000;
        const NEGATIVE      = 0b10000000000000000000000000000000;
    }
}

impl PSR {
    pub fn default() -> PSR {
        PSR::MODE_SVC
    }

    pub fn from(value: u32) -> PSR {
        PSR::from_bits_retain(value)
    }

    pub fn get_mode(&self) -> PSR {
        PSR::from_bits_truncate(self.bits() & PSR::MODE_MASK.bits())
    }
}

// P40
struct Byte(i8);
struct HalfWord(i16);
struct Word(i32);
struct UByte(u8);
struct UHalfWord(u16);
struct UWord(u32);
