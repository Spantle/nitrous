use core::mem::swap;

use super::PSR;

pub struct Arm9 {
    // R13: Stack Pointer
    // R14: Link Register
    // R15: Program Counter
    pub r: [u32; 16],    // registers
    pub r_fiq: [u32; 8], // r8-r14 + spsr
    pub r_irq: [u32; 3], // r13-r14 + spsr
    pub r_svc: [u32; 3], // r13-r14 + spsr
    pub r_abt: [u32; 3], // r13-r14 + spsr
    pub r_und: [u32; 3], // r13-r14 + spsr
    pub cpsr: PSR,       // Current Program Status Register, technically a u32
}

impl Arm9 {
    pub fn new() -> Arm9 {
        Arm9 {
            r: [0; 16],
            r_fiq: [0; 8],
            r_irq: [0; 3],
            r_svc: [0; 3],
            r_abt: [0; 3],
            r_und: [0; 3],
            cpsr: PSR::default(),
        }
    }

    fn switch_mode(&mut self, mode: PSR, is_error_or_interrupt: bool) {
        if mode.get_mode() == self.cpsr.get_mode() {
            return;
        }

        match mode {
            PSR::MODE_USR => {
                todo!("switch to user mode");
            }
            PSR::MODE_FIQ => {
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
                    PSR::MODE_IRQ => self.r_irq,
                    PSR::MODE_SVC => self.r_svc,
                    PSR::MODE_UND => self.r_und,
                    PSR::MODE_ABT => self.r_abt,
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
