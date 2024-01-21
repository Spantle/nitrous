use super::cpu::{arm9::Arm9, bus::Bus};

pub struct Emulator {
    pub arm9: Arm9,
    pub bus: Bus,

    running: bool,
}

impl Default for Emulator {
    fn default() -> Emulator {
        Emulator {
            arm9: Arm9::default(),
            bus: Bus::default(),

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

        self.arm9.clock(&mut self.bus);
    }

    pub fn step(&mut self) {
        self.arm9.step(&mut self.bus);
    }
}
