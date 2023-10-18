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

// Program Status Register (P31, P49)
pub struct PSR {
    negative: bool,      // N
    zero: bool,          // Z
    carry: bool,         // C
    overflow: bool,      // V
    sticky: bool,        // Q
    irq_interrupt: bool, // I
    fiq_interrupt: bool, // F
    thumb: bool,         // T
    mode: ProcessorMode,
}

impl PSR {
    pub fn new() -> PSR {
        PSR {
            negative: false,
            zero: false,
            carry: false,
            overflow: false,
            sticky: false,
            irq_interrupt: false,
            fiq_interrupt: false,
            thumb: false,
            mode: ProcessorMode::Usr,
        }
    }
}

// P41
enum ProcessorMode {
    Usr, // User       0b10000
    Fiq, // FIQ        0b10001
    Irq, // IRQ        0b10010
    Svc, // Supervisor 0b10011
    Abt, // Abort      0b10111
    Und, // Undefined  0b11011
    Sys, // System     0b11111
}

// P40
struct Byte(i8);
struct HalfWord(i16);
struct Word(i32);
struct UByte(u8);
struct UHalfWord(u16);
struct UWord(u32);
