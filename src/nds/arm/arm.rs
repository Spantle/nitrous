use crate::nds::{bus::BusTrait, cp15::CP15, logger, shared::Shared};

use super::{
    instructions::lookup_instruction_set,
    models::{Context, FakeDisassembly, ProcessorMode, Psr, Registers, StackTrace},
    ArmKind, ArmTrait,
};

pub struct Arm<Bus: BusTrait> {
    _phantom: std::marker::PhantomData<Bus>,

    pub halted: bool,

    // R13: Stack Pointer
    // R14: Link Register
    // R15: Program Counter
    pub r: Registers,    // registers
    pub r_fiq: [u32; 8], // r8-r14 + spsr
    pub r_irq: [u32; 3], // r13-r14 + spsr
    pub r_svc: [u32; 3], // r13-r14 + spsr
    pub r_abt: [u32; 3], // r13-r14 + spsr
    pub r_und: [u32; 3], // r13-r14 + spsr
    pub cpsr: Psr,       // Current Program Status Register, technically a u32

    // TODO: do this better
    // it's 2am i cannot be bothered
    // arm9 exclusives
    pub cp15: CP15,
    // arm7 exlusives
    pub wram7: Vec<u8>, // 64kb

    // emulator variables
    pub pc_changed: bool,
    pub stacktrace: StackTrace,
}

impl<Bus: BusTrait> Default for Arm<Bus> {
    fn default() -> Arm<Bus> {
        // TODO: in the future, the stack pointer MIGHT be set by the BIOS?
        let (sp, irq_sp, svc_sp) = if Bus::KIND == ArmKind::Arm9 {
            (0x00803EC0, 0x00803FA0, 0x00803FC0)
        } else {
            (0x0380FF00, 0x0380FFB0, 0x0380FFDC)
        };

        Arm::<Bus> {
            _phantom: std::marker::PhantomData,

            halted: false,

            r: Registers::new_with_sp(sp),
            r_fiq: [0, 0, 0, 0, 0, 0, 0, 0],
            r_irq: [irq_sp, 0, 0],
            r_svc: [svc_sp, 0, 0],
            r_abt: [0, 0, 0],
            r_und: [0, 0, 0],
            cpsr: Psr::default(),

            cp15: CP15::default(),
            wram7: vec![0; 1024 * 64],

            pc_changed: true,
            stacktrace: StackTrace::default(),
        }
    }
}

impl<Bus: BusTrait> Arm<Bus> {
    pub fn clock(&mut self, bus: &mut Bus, shared: &mut Shared) -> u32 {
        if self.halted {
            if !self.cpsr().get_irq_interrupt() && bus.is_requesting_interrupt() {
                self.handle_irq();
            }

            return 1; // TODO: how many cycles is a halted cpu lmao
        }

        let pc = self.r[15];
        let is_thumb = self.cpsr.get_thumb();
        let inst = if is_thumb {
            self.read_halfword(bus, shared, pc) as u32
        } else {
            self.read_word(bus, shared, pc)
        };
        // print as binary
        // if Bus::kind() == ArmKind::ARM7 {
        //     logger::debug(
        //         logger::LogSource::Arm7(self.r[15]),
        //         format!("executing instruction: {:#010X} ({:032b})", inst, inst),
        //     );
        // } else {
        //     logger::debug(
        //         logger::LogSource::Arm9(self.r[15]),
        //         format!("executing instruction: {:#010X} ({:032b})", inst, inst),
        //     );
        // }

        let mut cycles = match Bus::KIND {
            ArmKind::Arm9 => {
                lookup_instruction_set::<true>(&mut Context::new(
                    inst,
                    self,
                    bus,
                    shared,
                    &mut FakeDisassembly,
                    &mut logger::Logger(logger::LogSource::Arm9(inst)),
                )) + 2
            }
            ArmKind::Arm7 => lookup_instruction_set::<false>(&mut Context::new(
                inst,
                self,
                bus,
                shared,
                &mut FakeDisassembly,
                &mut logger::Logger(logger::LogSource::Arm7(inst)),
            )),
        };

        if !self.pc_changed {
            if is_thumb {
                self.r[15] += 2;
            } else {
                self.r[15] += 4;
            };
        } else {
            cycles += 2;

            self.stacktrace.branch(pc);

            let next_inst = if is_thumb {
                self.read_halfword(bus, shared, self.r[15]) as u32
            } else {
                self.read_word(bus, shared, self.r[15])
            };
            if next_inst == 0 {
                let log_source = if Bus::KIND == ArmKind::Arm9 {
                    logger::LogSource::Arm9(0)
                } else {
                    logger::LogSource::Arm7(0)
                };

                logger::error(
                    log_source,
                    self.stacktrace
                        .generate(self.r(), "PC sent to the shadow realm".to_string()),
                );
            }
        }

        self.pc_changed = false;

        if !self.cpsr().get_irq_interrupt() && bus.is_requesting_interrupt() {
            self.handle_irq();
        }

        cycles
    }

    fn handle_irq(&mut self) {
        self.set_mode_r(ProcessorMode::IRQ, 1, self.r[15] + 4);
        self.switch_mode::<false>(ProcessorMode::IRQ, true);
        self.cpsr_mut().set_thumb(false);
        self.cpsr_mut().set_irq_interrupt(true);

        if Bus::KIND == ArmKind::Arm9 {
            self.set_r(15, 0xFFFF0018);
        } else {
            self.set_r(15, 0x00000018);
        }

        self.halted = false;
    }
}
