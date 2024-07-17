use std::{fs, sync::atomic::AtomicBool};

use super::{
    cpu::arm::{
        bus::{bus7::Bus7, bus9::Bus9},
        Arm,
    },
    logger,
    shared::Shared,
};

static IS_EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub struct Emulator {
    pub arm9: Arm<Bus9>,
    pub arm7: Arm<Bus7>,

    pub bus9: Bus9,
    pub bus7: Bus7,

    pub shared: Shared,

    flipflop: bool,
}

impl Default for Emulator {
    fn default() -> Emulator {
        Emulator {
            arm9: Arm::default(),
            arm7: Arm::default(),

            bus9: Bus9::default(),
            bus7: Bus7::default(),

            shared: Shared::default(),

            flipflop: true,
        }
    }
}

impl Emulator {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // TODO: do some other resetting/initializing stuff here in the future
        let shared = &mut self.shared;
        let success = shared.cart.load(rom);
        if !success {
            return;
        }

        shared.psram = vec![0; 1024 * 1024 * 4];
        self.arm9 = Arm::default();
        self.arm7 = Arm::default();

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

    pub fn load_bios(&mut self, _bios: Vec<u8>) {
        // self.bus.arm9_bios = bios;
    }

    pub fn load_bios_from_path(&mut self, arm9_bios_path: &String) {
        let arm9 = fs::read(arm9_bios_path);
        match arm9 {
            Ok(arm9) => self.load_bios(arm9),
            Err(e) => {
                logger::error(
                    logger::LogSource::Emu,
                    format!("Failed to load ARM9 BIOS: {}", e),
                );
            }
        };
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

        // TODO: use cycles properly lol
        self.flipflop = !self.flipflop;
        if self.flipflop {
            self.arm7.clock(&mut self.bus7, &mut self.shared);
            self.shared.gpu2d_a.clock();
        } else {
            self.arm9.clock(&mut self.bus9, &mut self.shared);
        }
    }

    pub fn step(&mut self) {
        self.arm9.step(&mut self.bus9, &mut self.shared);
        self.arm7.step(&mut self.bus7, &mut self.shared);
    }
}

pub fn is_emulator_running() -> bool {
    IS_EMULATOR_RUNNING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_emulator_running(running: bool) {
    IS_EMULATOR_RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
}
