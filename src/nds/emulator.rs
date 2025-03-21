use std::sync::atomic::AtomicBool;

use crate::ui::windows::debug::arm::disassembler::ArmDisassemblerWindow;

use super::{
    arm::{Arm, ArmBool, ArmInternalRW},
    bus::{bus7::Bus7, bus9::Bus9, BusTrait},
    dma::Dma,
    logger::ONCE_LOGS,
    shared::Shared,
};

static IS_EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CycleState {
    Arm9_1,
    Arm9_2,
    Arm7,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Emulator {
    pub arm9: Arm<Bus9>,
    pub arm7: Arm<Bus7>,

    pub bus9: Bus9,
    pub bus7: Bus7,

    pub dma9: Dma,
    pub dma7: Dma,

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

            dma9: Dma::default(),
            dma7: Dma::default(),

            shared: Shared::default(),

            cycle_state: CycleState::Arm9_1,
        }
    }
}

impl Emulator {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset(false);

        let shared = &mut self.shared;
        let success = shared.cart.load(rom);
        if !success {
            return;
        }

        self.load_binary();
    }

    pub fn load_binary(&mut self) {
        let arm9_load_address = self.shared.cart.metadata.arm9_load_address;
        let arm9_bin = self.shared.cart.rom[self.shared.cart.metadata.arm9_rom_offset as usize
            ..(self.shared.cart.metadata.arm9_rom_offset + self.shared.cart.metadata.arm9_size)
                as usize]
            .to_vec();
        self.arm9.write_bulk(
            &mut self.bus9,
            &mut self.shared,
            &mut self.dma9,
            arm9_load_address,
            arm9_bin,
        );
        self.arm9.r[15] = self.shared.cart.metadata.arm9_entry_address;

        let arm7_load_address = self.shared.cart.metadata.arm7_load_address;
        let arm7_bin = self.shared.cart.rom[self.shared.cart.metadata.arm7_rom_offset as usize
            ..(self.shared.cart.metadata.arm7_rom_offset + self.shared.cart.metadata.arm7_size)
                as usize]
            .to_vec();
        self.arm7.write_bulk(
            &mut self.bus7,
            &mut self.shared,
            &mut self.dma7,
            arm7_load_address,
            arm7_bin,
        );
        self.arm7.r[15] = self.shared.cart.metadata.arm7_entry_address;

        // write game cartridge header into psram
        let shared = &mut self.shared;
        self.bus9.write_bulk(
            shared,
            &mut None,
            0x027FFE00,
            shared.cart.rom[0x0..0x200].into(),
        );

        // write chip id into psram
        self.bus9
            .write_word(shared, &mut None, 0x027FF800, 0x00000FC2);
        self.bus9
            .write_word(shared, &mut None, 0x027FF804, 0x00000FC2);
        self.bus9
            .write_word(shared, &mut None, 0x027FFC00, 0x00000FC2);
        self.bus9
            .write_word(shared, &mut None, 0x027FFC04, 0x00000FC2);

        // thanks @atem.zip
        self.bus9.write_byte(shared, &mut None, 0x04000247, 0x03);
        self.bus9
            .write_halfword(shared, &mut None, 0x027FF850, 0x5835);
        self.bus9
            .write_halfword(shared, &mut None, 0x027FF880, 0x0007);
        self.bus9
            .write_halfword(shared, &mut None, 0x027FF884, 0x0006);
        self.bus9
            .write_halfword(shared, &mut None, 0x027FFC10, 0x5835);
        self.bus9
            .write_halfword(shared, &mut None, 0x027FFC40, 0x0001);

        self.bus9.write_bulk(
            shared,
            &mut None,
            0x027FFC80,
            self.bus7.firmware[0x3FE00..0x3FE70].into(),
        );
    }

    pub fn load_state(&mut self, emulator: Emulator) {
        self.arm9 = emulator.arm9;
        self.arm7 = emulator.arm7;

        self.bus9.load_state(emulator.bus9);
        self.bus7.load_state(emulator.bus7);

        self.dma9 = emulator.dma9;
        self.dma7 = emulator.dma7;

        self.shared.load_state(emulator.shared);

        self.cycle_state = emulator.cycle_state;
    }

    pub fn reset(&mut self, load_binary: bool) {
        let was_running = self.is_running();
        self.pause();

        ONCE_LOGS.lock().unwrap().clear();

        self.arm9 = Arm::default();
        self.arm7 = Arm::default();
        self.bus9.reset();
        self.bus7.reset();
        self.dma9 = Dma::default();
        self.dma7 = Dma::default();
        self.shared.reset();

        if load_binary {
            self.load_binary();
        }

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
                let cycles = self
                    .arm9
                    .clock(&mut self.bus9, &mut self.shared, &mut self.dma9);
                self.dma9
                    .check_immediately(&mut self.bus9, &mut self.shared);
                self.shared.cart.clock(&mut self.bus9, &mut self.bus7);

                cycles
            }
            CycleState::Arm7 => {
                let cycles = self
                    .arm7
                    .clock(&mut self.bus7, &mut self.shared, &mut self.dma7);
                self.shared.gpus.clock(&mut self.bus9, &mut self.bus7);
                self.dma7
                    .check_immediately(&mut self.bus7, &mut self.shared);

                self.bus9.timers.clock(cycles, &mut self.bus9.interrupts);
                self.bus7.timers.clock(cycles, &mut self.bus7.interrupts);
                self.bus9.div.clock(cycles);
                self.bus9.sqrt.clock();

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

            let arm9_cycles = self
                .arm9
                .clock(&mut self.bus9, &mut self.shared, &mut self.dma9);

            cycles_ran_arm9 += arm9_cycles as u64;

            let target_cycles_arm7 = (cycles_ran_arm9 / 2) as i32;
            let target_cycles_gpu = cycles_ran_arm9 / 2;

            while cycles_ran_arm7 < target_cycles_arm7 {
                if !self.is_running() {
                    // TODO: the arm7 can fall behind if we stop it early and continue everything else. not sure if it matters
                    break;
                }

                let arm7_cycles = self
                    .arm7
                    .clock(&mut self.bus7, &mut self.shared, &mut self.dma7);
                cycles_ran_arm7 += arm7_cycles as i32;

                self.bus9
                    .timers
                    .clock(arm7_cycles, &mut self.bus9.interrupts);
                self.bus7
                    .timers
                    .clock(arm7_cycles, &mut self.bus7.interrupts);
                self.bus9.div.clock(arm7_cycles);
                self.bus9.sqrt.clock();

                disassembler_windows
                    .1
                    .check_breakpoints::<{ ArmBool::ARM7 }>(self);
            }

            while cycles_ran_gpu < target_cycles_gpu {
                self.shared.gpus.clock(&mut self.bus9, &mut self.bus7);
                cycles_ran_gpu += 1;
            }

            self.dma9
                .check_immediately(&mut self.bus9, &mut self.shared);
            self.dma7
                .check_immediately(&mut self.bus7, &mut self.shared);

            self.shared
                .ipcsync
                .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);
            self.shared
                .ipcfifo
                .update_interrupts(&mut self.bus9.interrupts, &mut self.bus7.interrupts);

            self.shared.cart.clock(&mut self.bus9, &mut self.bus7);

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
