use crate::arm;

pub struct Emulator {
    pub arm9: arm::Arm9,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            arm9: arm::Arm9::new(),
        }
    }
}
