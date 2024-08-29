use std::sync::atomic::AtomicBool;

use super::{
    arm::{
        bus::{bus7::Bus7, bus9::Bus9},
        Arm,
    },
    shared::Shared,
};

static IS_EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(PartialEq)]
pub enum CycleState {
    Arm9_1,
    Arm9_2,
    Arm7,
}

pub struct Emulator {
    pub arm9: Arm<Bus9>,
    pub arm7: Arm<Bus7>,

    pub bus9: Bus9,
    pub bus7: Bus7,

    pub shared: Shared,

    pub cycle_state: CycleState,
}

impl Default for Emulator {
    fn default() -> Emulator {
        Emulator {
            arm9: Arm::default(),
            arm7: Arm::default(),

            bus9: Bus9::default(),
            bus7: Bus7::default(),

            shared: Shared::default(),

            cycle_state: CycleState::Arm9_1,
        }
    }
}

impl Emulator {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset();

        let shared = &mut self.shared;
        let success = shared.cart.load(rom);
        if !success {
            return;
        }

        self.load_binary();
    }

    pub fn load_binary(&mut self) {
        let arm9_load_address = self.shared.cart.arm9_load_address;
        let arm9_bin = self.shared.cart.rom[self.shared.cart.arm9_rom_offset as usize
            ..(self.shared.cart.arm9_rom_offset + self.shared.cart.arm9_size) as usize]
            .to_vec();
        self.arm9.write_bulk(
            &mut self.bus9,
            &mut self.shared,
            arm9_load_address,
            arm9_bin,
        );
        self.arm9.r[15] = self.shared.cart.arm9_entry_address;

        let arm7_load_address = self.shared.cart.arm7_load_address;
        let arm7_bin = self.shared.cart.rom[self.shared.cart.arm7_rom_offset as usize
            ..(self.shared.cart.arm7_rom_offset + self.shared.cart.arm7_size) as usize]
            .to_vec();
        self.arm7.write_bulk(
            &mut self.bus7,
            &mut self.shared,
            arm7_load_address,
            arm7_bin,
        );
        self.arm7.r[15] = self.shared.cart.arm7_entry_address;
    }

    pub fn reset(&mut self) {
        let was_running = self.is_running();
        self.pause();

        self.arm9 = Arm::default();
        self.arm7 = Arm::default();

        self.shared.reset();

        self.load_binary();

        if was_running {
            self.start();
        }
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

    // NOTE: do not use this in a loop, it is slow
    pub fn step(&mut self) -> u32 {
        match self.cycle_state {
            CycleState::Arm9_1 => {
                let cycles = self.arm9.clock(&mut self.bus9, &mut self.shared);
                self.shared.dma9 = self
                    .shared
                    .dma9
                    .clone()
                    .check_immediately(&mut self.bus9, &mut self.shared);

                self.cycle_state = CycleState::Arm9_2;
                cycles
            }
            CycleState::Arm9_2 => {
                let cycles = self.arm9.clock(&mut self.bus9, &mut self.shared);
                self.shared.dma9 = self
                    .shared
                    .dma9
                    .clone()
                    .check_immediately(&mut self.bus9, &mut self.shared);

                self.cycle_state = CycleState::Arm7;
                cycles
            }
            CycleState::Arm7 => {
                let cycles = self.arm7.clock(&mut self.bus7, &mut self.shared);
                self.shared.gpu2d_a.clock(&mut self.bus9, &mut self.bus7);
                self.shared.gpu2d_b.clock(&mut self.bus9, &mut self.bus7);
                self.shared.dma7 = self
                    .shared
                    .dma7
                    .clone()
                    .check_immediately(&mut self.bus7, &mut self.shared);

                self.cycle_state = CycleState::Arm9_1;
                cycles
            }
        }
    }
}

pub fn is_emulator_running() -> bool {
    IS_EMULATOR_RUNNING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_emulator_running(running: bool) {
    IS_EMULATOR_RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
}
