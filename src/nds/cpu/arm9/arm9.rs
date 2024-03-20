use core::mem::swap;

use crate::nds::{
    cpu::{
        arm9::instructions::lookup_instruction_set,
        bus::{Bus, BusTrait},
    },
    logger,
};

use super::models::{Context, FakeDisassembly, PipelineState, ProcessorMode, Registers, PSR};

#[derive(Debug)]
pub struct Arm9 {
    // R13: Stack Pointer
    // R14: Link Register
    // R15: Program Counter
    pub r: Registers,    // registers
    pub r_fiq: [u32; 8], // r8-r14 + spsr
    pub r_irq: [u32; 3], // r13-r14 + spsr
    pub r_svc: [u32; 3], // r13-r14 + spsr
    pub r_abt: [u32; 3], // r13-r14 + spsr
    pub r_und: [u32; 3], // r13-r14 + spsr
    pub cpsr: PSR,       // Current Program Status Register, technically a u32

    // emulator variables
    pub pipeline_state: PipelineState,
}

pub trait Arm9Trait {
    fn r(&mut self) -> &mut Registers; // TODO: rename this to `registers`
    fn er(&self, r: u8) -> u32;
    fn eru(&self, r: u8) -> u32;

    fn cpsr(&mut self) -> &mut PSR;
    fn set_cpsr(&mut self, psr: PSR);
    fn get_spsr(&self) -> PSR;
    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        mode: ProcessorMode,
        copy_cpsr_to_spsr: bool,
    );
}

impl Default for Arm9 {
    fn default() -> Arm9 {
        Arm9 {
            r: Registers::default(),
            r_fiq: [0, 0, 0, 0, 0, 0, 0, 0],
            r_irq: [0x803FA0, 0, 0], // TODO: in the future, the stack pointer should be set by the BIOS
            r_svc: [0x803FC0, 0, 0], // TODO: in the future, the stack pointer should be set by the BIOS
            r_abt: [0, 0, 0],
            r_und: [0, 0, 0],
            cpsr: PSR::default(),

            pipeline_state: PipelineState::Fetch,
        }
    }
}

impl Arm9 {
    pub fn clock(&mut self, bus: &mut Bus) -> bool {
        match self.pipeline_state {
            PipelineState::Fetch => {
                // logger::debug(logger::LogSource::Arm9, "fetching instruction");
                self.pipeline_state = PipelineState::Decode;
                false
            }
            PipelineState::Decode => {
                // logger::debug(logger::LogSource::Arm9, "decoding instruction");
                self.pipeline_state = PipelineState::Execute;
                false
            }
            PipelineState::Execute => {
                // get 4 bytes
                let inst = bus.read_word(self.r[15]);
                // print as binary
                // logger::debug(
                //     logger::LogSource::Arm9,
                //     format!("executing instruction: {:#010X} ({:032b})", inst, inst),
                // );

                let r15 = self.r[15];
                let cycles = lookup_instruction_set(&mut Context::new(
                    inst.into(),
                    self,
                    bus,
                    &mut FakeDisassembly,
                    &mut logger::Logger(logger::LogSource::Arm9(inst)),
                ));
                if r15 == self.r[15] {
                    self.r[15] += 4;
                } else {
                    self.pipeline_state = PipelineState::Fetch;
                }

                true
            }
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        loop {
            if self.clock(bus) {
                break;
            }
        }
    }
}

impl Arm9Trait for Arm9 {
    fn r(&mut self) -> &mut Registers {
        &mut self.r
    }

    // this stands for "get execute register"
    // when executing instructions, the PC is 8 bytes ahead of the current instruction
    fn er(&self, r: u8) -> u32 {
        match r {
            15 => self.r[15] + 8,
            _ => self.r[r],
        }
    }

    // this stands for "get execute register unpredictable"
    // when executing instructions, the PC is unpredictable
    fn eru(&self, r: u8) -> u32 {
        match r {
            15 => {
                logger::warn(
                    logger::LogSource::Arm9(0),
                    "UNPREDICTABLE: r15 was specified in an invalid context",
                );
                self.r[15] // NOTE: this might need to be + 8?
            }
            _ => self.r[r],
        }
    }

    fn cpsr(&mut self) -> &mut PSR {
        &mut self.cpsr
    }

    // TODO: review if setting the cpsr also changes modes?
    fn set_cpsr(&mut self, psr: PSR) {
        // let new_mode = psr.get_mode();
        // if new_mode != self.cpsr.get_mode() {
        //     self.switch_mode::<false>(new_mode, false);
        // }

        self.cpsr = psr;
    }

    fn get_spsr(&self) -> PSR {
        match self.cpsr.get_mode() {
            ProcessorMode::FIQ => PSR::from(self.r_fiq[7]),
            ProcessorMode::IRQ => PSR::from(self.r_irq[2]),
            ProcessorMode::SVC => PSR::from(self.r_svc[2]),
            ProcessorMode::ABT => PSR::from(self.r_abt[2]),
            ProcessorMode::UND => PSR::from(self.r_und[2]),
            _ => {
                logger::warn(
                    logger::LogSource::Arm9(0),
                    "UNPREDICTABLE: attempt to get SPSR in non-exception mode.",
                );
                PSR::default()
            }
        }
    }

    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        mode: ProcessorMode,
        copy_cpsr_to_spsr: bool,
    ) {
        let current_mode = self.cpsr.get_mode();
        let mut new_mode = mode;

        if RETURN_TO_DEFAULT {
            // put things back the way that they were
            new_mode = current_mode;
        } else {
            if current_mode == new_mode {
                return;
            }

            if self.cpsr.current_mode_has_spsr() {
                self.switch_mode::<true>(ProcessorMode::USR, false);
            }

            // a cheat to invert the swapping logic without having to copy paste a bunch of code >:)
            if new_mode == ProcessorMode::SYS || new_mode == ProcessorMode::USR {
                new_mode = current_mode;
            }
        }

        match new_mode {
            ProcessorMode::FIQ => {
                swap(&mut self.r[8], &mut self.r_fiq[0]);
                swap(&mut self.r[9], &mut self.r_fiq[1]);
                swap(&mut self.r[10], &mut self.r_fiq[2]);
                swap(&mut self.r[11], &mut self.r_fiq[3]);
                swap(&mut self.r[12], &mut self.r_fiq[4]);
                swap(&mut self.r[13], &mut self.r_fiq[5]);
                swap(&mut self.r[14], &mut self.r_fiq[6]);
                if copy_cpsr_to_spsr {
                    self.r_fiq[7] = self.cpsr.value();
                }
            }
            _ => {
                let r_to_swap = &mut match new_mode {
                    ProcessorMode::IRQ => self.r_irq,
                    ProcessorMode::SVC => self.r_svc,
                    ProcessorMode::UND => self.r_und,
                    ProcessorMode::ABT => self.r_abt,
                    _ => return,
                };

                swap(&mut self.r[13], &mut r_to_swap[0]);
                swap(&mut self.r[14], &mut r_to_swap[1]);
                if copy_cpsr_to_spsr {
                    r_to_swap[2] = self.cpsr.value();
                }
            }
        }
    }
}

pub struct FakeArm9 {
    r: Registers,
    cpsr: PSR,
}

impl FakeArm9 {
    pub fn new(r15: u32) -> FakeArm9 {
        FakeArm9 {
            r: Registers::new(r15 + 8),
            cpsr: PSR::default(),
        }
    }
}

impl Arm9Trait for FakeArm9 {
    fn r(&mut self) -> &mut Registers {
        &mut self.r
    }

    fn er(&self, r: u8) -> u32 {
        self.r[r]
    }

    fn eru(&self, r: u8) -> u32 {
        self.r[r]
    }

    fn cpsr(&mut self) -> &mut PSR {
        &mut self.cpsr
    }

    fn set_cpsr(&mut self, _psr: PSR) {}

    fn get_spsr(&self) -> PSR {
        PSR::default()
    }

    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        _mode: ProcessorMode,
        _copy_cpsr_to_spsr: bool,
    ) {
    }
}
