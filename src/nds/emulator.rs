use super::cpu::arm9::Arm9;

pub struct Emulator {
    pub arm9: Arm9,
    pub mem: Vec<u8>,

    running: bool,
}

impl Default for Emulator {
    fn default() -> Emulator {
        let mut mem = vec![0; 1024 * 1024 * 4];
        // poor example to fill memory
        mem[0] = 0b00001010;
        mem[1] = 0b00010000;
        mem[2] = 0b10100000;
        mem[3] = 0b11100011;

        Emulator {
            arm9: Arm9::default(),
            mem,

            running: false,
        }
    }
}

impl Emulator {
    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn clock(&mut self) {
        if !self.running {
            return;
        }

        self.arm9.clock(&mut self.mem);
    }
}
