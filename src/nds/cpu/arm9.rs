use core::mem::swap;

use crate::nds::{cpu::arm9::instructions::lookup_instruction_set, cpu::bus::Bus, logger};

use self::models::{PipelineState, Registers};
pub use self::models::{ProcessorMode, PSR};

mod instructions;
mod models;

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

impl Default for Arm9 {
    fn default() -> Arm9 {
        // TODO: figure out what the default PSR value is for SPSRs
        let psr = PSR::default().value();
        Arm9 {
            r: Registers([0; 16]),
            r_fiq: [0, 0, 0, 0, 0, 0, 0, psr],
            r_irq: [0, 0, psr],
            r_svc: [0, 0, psr],
            r_abt: [0, 0, psr],
            r_und: [0, 0, psr],
            cpsr: PSR::default(),

            pipeline_state: PipelineState::Fetch,
        }
    }
}

impl Arm9 {
    pub fn clock(&mut self, bus: &mut Bus) -> bool {
        match self.pipeline_state {
            PipelineState::Fetch => {
                logger::debug("fetching instruction");
                self.pipeline_state = PipelineState::Decode;
                false
            }
            PipelineState::Decode => {
                logger::debug("decoding instruction");
                self.pipeline_state = PipelineState::Execute;
                false
            }
            PipelineState::Execute => {
                // get 4 bytes
                let inst = bus.read_word(self.r[15]);
                // print as binary
                logger::debug(format!(
                    "executing instruction: {:#08X} ({:032b})",
                    inst, inst
                ));

                let r15 = self.r[15];
                let cycles = lookup_instruction_set(inst.into(), self, bus);
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

    pub fn get_spsr(&self) -> PSR {
        match self.cpsr.get_mode() {
            ProcessorMode::FIQ => PSR::from(self.r_fiq[7]),
            ProcessorMode::IRQ => PSR::from(self.r_irq[2]),
            ProcessorMode::SVC => PSR::from(self.r_svc[2]),
            ProcessorMode::ABT => PSR::from(self.r_abt[2]),
            ProcessorMode::UND => PSR::from(self.r_und[2]),
            _ => {
                logger::warn("UNPREDICTABLE: attempt to get SPSR in non-exception mode.");
                PSR::default()
            }
        }
    }

    // this stands for "get execute register"
    // when executing instructions, the PC is 8 bytes ahead of the current instruction
    pub fn er(&self, r: u8) -> u32 {
        match r {
            15 => self.r[15] + 8,
            _ => self.r[r],
        }
    }

    // this stands for "get execute register unpredictable"
    // when executing instructions, the PC is unpredictable
    pub fn eru(&self, r: u8) -> u32 {
        match r {
            15 => {
                logger::warn("UNPREDICTABLE: r15 was specified in an invalid context");
                self.r[15] // NOTE: this might need to be + 8?
            }
            _ => self.r[r],
        }
    }

    fn switch_mode(&mut self, mode: ProcessorMode, is_error_or_interrupt: bool) {
        if self.cpsr.get_mode() == mode {
            return;
        }

        match mode {
            ProcessorMode::USR => {
                todo!("switch to user mode");
            }
            ProcessorMode::FIQ => {
                swap(&mut self.r[8], &mut self.r_fiq[0]);
                swap(&mut self.r[9], &mut self.r_fiq[1]);
                swap(&mut self.r[10], &mut self.r_fiq[2]);
                swap(&mut self.r[11], &mut self.r_fiq[3]);
                swap(&mut self.r[12], &mut self.r_fiq[4]);
                swap(&mut self.r[13], &mut self.r_fiq[5]);
                swap(&mut self.r[14], &mut self.r_fiq[6]);
                if is_error_or_interrupt {
                    swap(&mut self.cpsr, &mut PSR::from(self.r_fiq[7]));
                }
            }
            _ => {
                let r_to_swap = &mut match mode {
                    ProcessorMode::IRQ => self.r_irq,
                    ProcessorMode::SVC => self.r_svc,
                    ProcessorMode::UND => self.r_und,
                    ProcessorMode::ABT => self.r_abt,
                    _ => return,
                };

                swap(&mut self.r[13], &mut r_to_swap[0]);
                swap(&mut self.r[14], &mut r_to_swap[1]);
                if is_error_or_interrupt {
                    swap(&mut self.cpsr, &mut PSR::from(r_to_swap[2]));
                }
            }
        }
    }
}
