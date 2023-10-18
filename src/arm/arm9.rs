use crate::arm::models::PSR;

pub struct Arm9 {
    // R0-R7: Unbanked 32-bit general purpose registers
    // R8-R14: Banked registers (they do something?) (P44)
    // R13: Stack Pointer
    // R14: Link Register
    // R15: Program Counter
    pub registers: [u32; 16], // general purpose registers (there's supposed to be 31)
    // there are also 6 status registers (P42)
    // there's some weird diagram about register arrangement on P43
    pub cpsr: PSR,
}

impl Arm9 {
    pub fn new() -> Arm9 {
        Arm9 {
            registers: [0; 16],
            cpsr: PSR::new(),
        }
    }
}
