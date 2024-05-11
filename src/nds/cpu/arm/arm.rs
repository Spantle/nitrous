use core::mem::swap;

use crate::nds::{
    cpu::{
        arm::instructions::lookup_instruction_set,
        bus::{Bus, BusTrait},
    },
    logger,
};

use super::models::{Context, FakeDisassembly, PipelineState, ProcessorMode, Registers, PSR};

pub enum ArmKind {
    ARM9,
    ARM7,
}
impl ArmKind {
    #[inline(always)]
    pub fn from_bool(arm_bool: bool) -> ArmKind {
        match arm_bool {
            true => ArmKind::ARM9,
            false => ArmKind::ARM7,
        }
    }
}
pub struct ArmBool;
impl ArmBool {
    pub const ARM9: bool = true;
    pub const ARM7: bool = false;
}

#[derive(Debug)]
pub struct Arm<const ARM_BOOL: bool> {
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
    pub pc_changed: bool,
}

pub trait ArmTrait {
    fn r(&self) -> &Registers; // TODO: rename this to `registers`
    fn set_r(&mut self, r: u8, value: u32);
    fn er(&self, r: u8) -> u32;
    fn eru(&self, r: u8) -> u32;

    fn cpsr(&self) -> &PSR;
    fn cpsr_mut(&mut self) -> &mut PSR;
    fn set_cpsr(&mut self, psr: PSR);
    fn get_spsr(&self) -> PSR;
    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        mode: ProcessorMode,
        copy_cpsr_to_spsr: bool,
    );
}

impl<const ARM_BOOL: bool> Default for Arm<ARM_BOOL> {
    fn default() -> Arm<ARM_BOOL> {
        Arm {
            r: Registers::default(),
            r_fiq: [0, 0, 0, 0, 0, 0, 0, 0],
            r_irq: [0x803FA0, 0, 0], // TODO: in the future, the stack pointer should be set by the BIOS
            r_svc: [0x803FC0, 0, 0], // TODO: in the future, the stack pointer should be set by the BIOS
            r_abt: [0, 0, 0],
            r_und: [0, 0, 0],
            cpsr: PSR::default(),

            pipeline_state: PipelineState::Fetch,
            pc_changed: false,
        }
    }
}

impl<const ARM_BOOL: bool> Arm<ARM_BOOL> {
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

                let cycles = match ARM_BOOL {
                    ArmBool::ARM9 => lookup_instruction_set::<true>(&mut Context::new(
                        inst.into(),
                        self,
                        bus,
                        &mut FakeDisassembly,
                        &mut logger::Logger(logger::LogSource::Arm9(inst)),
                    )),
                    ArmBool::ARM7 => lookup_instruction_set::<false>(&mut Context::new(
                        inst.into(),
                        self,
                        bus,
                        &mut FakeDisassembly,
                        &mut logger::Logger(logger::LogSource::Arm7(inst)),
                    )),
                };

                if !self.pc_changed {
                    self.r[15] += 4;
                } else {
                    self.pc_changed = false;
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

impl<const ARM_BOOL: bool> ArmTrait for Arm<ARM_BOOL> {
    fn r(&self) -> &Registers {
        &self.r
    }

    fn set_r(&mut self, r: u8, value: u32) {
        self.pc_changed = self.pc_changed || r == 15;
        self.r[r] = value;
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
                let log_source = match ARM_BOOL {
                    ArmBool::ARM9 => logger::LogSource::Arm9(0),
                    ArmBool::ARM7 => logger::LogSource::Arm7(0),
                };
                logger::warn(
                    log_source,
                    "UNPREDICTABLE: r15 was specified in an invalid context",
                );
                self.r[15] // NOTE: this might need to be + 8?
            }
            _ => self.r[r],
        }
    }

    fn cpsr(&self) -> &PSR {
        &self.cpsr
    }

    fn cpsr_mut(&mut self) -> &mut PSR {
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
                let log_source = match ARM_BOOL {
                    ArmBool::ARM9 => logger::LogSource::Arm9(0),
                    ArmBool::ARM7 => logger::LogSource::Arm7(0),
                };
                logger::warn(
                    log_source,
                    "UNPREDICTABLE: attempt to get SPSR in non-exception mode.",
                );
                PSR::default()
            }
        }
    }

    // RETURN_TO_DEFAULT should be false, it should only be used by this function internally
    // copy_cpsr_to_spsr should be true only when an exception switches the mode
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

            // put things back before switching to the new mode
            if self.cpsr.current_mode_has_spsr() {
                self.switch_mode::<true>(ProcessorMode::USR, false);
            }

            // a cheat to invert the swapping logic without having to copy paste a bunch of code >:)
            // update: i don't know what this was for
            // if new_mode == ProcessorMode::SYS || new_mode == ProcessorMode::USR {
            //     new_mode = current_mode;
            // }
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
                    ProcessorMode::IRQ => &mut self.r_irq,
                    ProcessorMode::SVC => &mut self.r_svc,
                    ProcessorMode::UND => &mut self.r_und,
                    ProcessorMode::ABT => &mut self.r_abt,
                    _ => {
                        self.cpsr.set_mode(new_mode);
                        return;
                    }
                };

                swap(&mut self.r[13], &mut r_to_swap[0]);
                swap(&mut self.r[14], &mut r_to_swap[1]);
                if copy_cpsr_to_spsr {
                    r_to_swap[2] = self.cpsr.value();
                }
            }
        }

        self.cpsr.set_mode(new_mode);
    }
}

pub struct FakeArm {
    r: Registers,
    cpsr: PSR,
}

impl FakeArm {
    pub fn new(r15: u32) -> FakeArm {
        FakeArm {
            r: Registers::new(r15 + 8),
            cpsr: PSR::default(),
        }
    }
}

impl ArmTrait for FakeArm {
    fn r(&self) -> &Registers {
        &self.r
    }

    fn set_r(&mut self, r: u8, value: u32) {
        self.r[r] = value;
    }

    fn er(&self, r: u8) -> u32 {
        self.r[r]
    }

    fn eru(&self, r: u8) -> u32 {
        self.r[r]
    }

    fn cpsr(&self) -> &PSR {
        &self.cpsr
    }

    fn cpsr_mut(&mut self) -> &mut PSR {
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
