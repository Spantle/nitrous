use std::sync::atomic::AtomicBool;

use super::cpu::{
    arm9::Arm9,
    bus::{Bus, BusTrait},
};

static IS_EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub struct Emulator {
    pub arm9: Arm9,
    pub bus: Bus,
}

impl Default for Emulator {
    fn default() -> Emulator {
        Emulator {
            arm9: Arm9::default(),
            bus: Bus::default(),
        }
    }
}

impl Emulator {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // TODO: do some other resetting/initializing stuff here in the future
        let success = self.bus.cart.load(rom);
        if !success {
            return;
        }

        self.bus.mem = vec![0; 1024 * 1024 * 4];

        self.bus.write_bulk(
            self.bus.cart.arm9_load_address,
            self.bus.cart.rom[self.bus.cart.arm9_rom_offset as usize
                ..(self.bus.cart.arm9_rom_offset + self.bus.cart.arm9_size) as usize]
                .to_vec(),
        );

        self.arm9 = Arm9::default();
        self.arm9.r[15] = self.bus.cart.arm9_entry_address;
    }

    pub fn start(&mut self) {
        set_emulator_running(true);
    }

    pub fn pause(&mut self) {
        set_emulator_running(false);
    }

    pub fn is_running(&self) -> bool {
        is_emulator_running()
    }

    pub fn clock(&mut self) {
        if !self.is_running() {
            return;
        }

        self.arm9.clock(&mut self.bus);
    }

    pub fn step(&mut self) {
        self.arm9.step(&mut self.bus);
    }
}

pub fn is_emulator_running() -> bool {
    IS_EMULATOR_RUNNING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_emulator_running(running: bool) {
    IS_EMULATOR_RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
}
