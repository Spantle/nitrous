use std::sync::atomic::AtomicBool;

use crate::ui::windows::debug::arm::disassembler::ArmDisassemblerWindow;

use super::{
    arm::{Arm, ArmBool, ArmInternalRW},
    bus::{bus7::Bus7, bus9::Bus9, BusTrait},
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
        self.bus9.reset();
        self.bus7.reset();

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
        let cycles = match self.cycle_state {
            CycleState::Arm9_1 | CycleState::Arm9_2 => {
                let cycles = self.arm9.clock(&mut self.bus9, &mut self.shared);
                self.bus9.dma = self
                    .bus9
                    .dma
                    .clone()
                    .check_immediately(&mut self.bus9, &mut self.shared);

                cycles
            }
            CycleState::Arm7 => {
                let cycles = self.arm7.clock(&mut self.bus7, &mut self.shared);
                self.shared.gpus.clock(&mut self.bus9, &mut self.bus7);
                self.bus7.dma = self
                    .bus7
                    .dma
                    .clone()
                    .check_immediately(&mut self.bus7, &mut self.shared);

                cycles
            }
        };

        self.shared
            .ipcsync
            .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);
        self.shared
            .ipcfifo
            .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);

        self.cycle_state = match self.cycle_state {
            CycleState::Arm9_1 => CycleState::Arm9_2,
            CycleState::Arm9_2 => CycleState::Arm7,
            CycleState::Arm7 => CycleState::Arm9_1,
        };

        cycles
    }

    pub fn run_for(
        &mut self,
        target_cycles_arm9: u64,
        last_cycle_arm7_discrepency: i32,
        disassembler_windows: (&mut ArmDisassemblerWindow, &mut ArmDisassemblerWindow),
    ) -> (u64, i32, u64) {
        let mut cycles_ran_arm9 = 0;
        let mut cycles_ran_arm7 = last_cycle_arm7_discrepency;
        let mut cycles_ran_gpu = 0;

        if !self.is_running() {
            return (0, last_cycle_arm7_discrepency, 0);
        }

        while cycles_ran_arm9 < target_cycles_arm9 {
            if !self.is_running() {
                break;
            }

            let arm9_cycles = self.arm9.clock(&mut self.bus9, &mut self.shared);

            cycles_ran_arm9 += arm9_cycles as u64;

            let target_cycles_arm7 = (cycles_ran_arm9 / 2) as i32;
            let target_cycles_gpu = cycles_ran_arm9 / 2;

            while cycles_ran_arm7 < target_cycles_arm7 {
                if !self.is_running() {
                    // TODO: the arm7 can fall behind if we stop it early and continue everything else. not sure if it matters
                    break;
                }

                let arm7_cycles = self.arm7.clock(&mut self.bus7, &mut self.shared);
                cycles_ran_arm7 += arm7_cycles as i32;

                disassembler_windows
                    .1
                    .check_breakpoints::<{ ArmBool::ARM7 }>(self);
            }

            while cycles_ran_gpu < target_cycles_gpu {
                self.shared.gpus.clock(&mut self.bus9, &mut self.bus7);
                cycles_ran_gpu += 1;
            }

            // this sucks lmao
            self.bus9.dma = self
                .bus9
                .dma
                .clone()
                .check_immediately(&mut self.bus9, &mut self.shared);
            self.bus7.dma = self
                .bus7
                .dma
                .clone()
                .check_immediately(&mut self.bus7, &mut self.shared);

            self.shared
                .ipcsync
                .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);
            self.shared
                .ipcfifo
                .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);

            disassembler_windows
                .0
                .check_breakpoints::<{ ArmBool::ARM9 }>(self);
        }

        (cycles_ran_arm9, cycles_ran_arm7, cycles_ran_gpu)
    }
}

pub fn is_emulator_running() -> bool {
    IS_EMULATOR_RUNNING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_emulator_running(running: bool) {
    IS_EMULATOR_RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
}
