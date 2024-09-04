use core::mem::swap;

use crate::nds::{bus::BusTrait, cp15::CP15, logger, shared::Shared};

use super::{
    models::{ProcessorMode, Registers, PSR},
    Arm, ArmInternalRW, ArmKind,
};

pub trait ArmTrait<Bus: BusTrait> {
    // NOTE: do not use this if there's a possibility that the PC is being read
    fn r(&self) -> &Registers;
    fn set_r(&mut self, r: u8, value: u32);
    fn er(&self, r: u8) -> u32;
    fn ert(&self, r: u8) -> u32;
    fn eru(&self, r: u8) -> u32;
    fn set_mode_r(&mut self, mode: ProcessorMode, r: u8, value: u32);

    fn cpsr(&self) -> &PSR;
    fn cpsr_mut(&mut self) -> &mut PSR;
    fn set_cpsr(&mut self, psr: PSR);
    fn get_spsr(&self) -> PSR;
    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        mode: ProcessorMode,
        copy_cpsr_to_spsr: bool,
    );

    fn cp15(&self) -> &CP15;
    fn cp15_mut(&mut self) -> &mut CP15;

    fn read_byte(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u8;
    fn read_halfword(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u16;
    fn read_word(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u32;

    fn write_byte(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u8);
    fn write_halfword(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u16);
    fn write_word(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u32);
}

impl<Bus: BusTrait> ArmTrait<Bus> for Arm<Bus> {
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

    // this stands for "get execute register thumb"
    // when executing instructions, the PC is 4 bytes ahead of the current instruction
    fn ert(&self, r: u8) -> u32 {
        match r {
            15 => self.r[15] + 4,
            _ => self.r[r],
        }
    }

    // this stands for "get execute register unpredictable"
    // when executing instructions, the PC is unpredictable
    fn eru(&self, r: u8) -> u32 {
        match r {
            15 => {
                let log_source = match Bus::KIND {
                    ArmKind::Arm9 => logger::LogSource::Arm9(0),
                    ArmKind::Arm7 => logger::LogSource::Arm7(0),
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

    fn set_mode_r(&mut self, mode: ProcessorMode, r: u8, value: u32) {
        match mode {
            ProcessorMode::FIQ => self.r_fiq[r as usize] = value,
            ProcessorMode::IRQ => self.r_irq[r as usize] = value,
            ProcessorMode::SVC => self.r_svc[r as usize] = value,
            ProcessorMode::ABT => self.r_abt[r as usize] = value,
            ProcessorMode::UND => self.r_und[r as usize] = value,
            _ => self.r[r] = value,
        }
    }

    fn cpsr(&self) -> &PSR {
        &self.cpsr
    }

    fn cpsr_mut(&mut self) -> &mut PSR {
        &mut self.cpsr
    }

    fn set_cpsr(&mut self, psr: PSR) {
        let new_mode = psr.get_mode();
        if new_mode != self.cpsr.get_mode() {
            self.switch_mode::<false>(new_mode, false);
        }

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
                let log_source = match Bus::KIND {
                    ArmKind::Arm9 => logger::LogSource::Arm9(0),
                    ArmKind::Arm7 => logger::LogSource::Arm7(0),
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

    fn cp15(&self) -> &CP15 {
        &self.cp15
    }

    fn cp15_mut(&mut self) -> &mut CP15 {
        &mut self.cp15
    }

    fn read_byte(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u8 {
        self.read_slice::<1>(bus, shared, addr)[0]
    }
    fn read_halfword(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u16 {
        let bytes = self.read_slice::<2>(bus, shared, addr);
        u16::from_le_bytes(bytes)
    }
    fn read_word(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u32 {
        let bytes = self.read_slice::<4>(bus, shared, addr);
        u32::from_le_bytes(bytes)
    }

    fn write_byte(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u8) {
        self.write_slice::<1>(bus, shared, addr, [value]);
    }
    fn write_halfword(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u16) {
        self.write_slice::<2>(bus, shared, addr, value.to_le_bytes());
    }
    fn write_word(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u32) {
        self.write_slice::<4>(bus, shared, addr, value.to_le_bytes());
    }
}
