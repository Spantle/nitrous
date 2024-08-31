use core::mem::swap;

use crate::nds::{cp15::CP15, logger, shared::Shared};

use super::{
    bus::BusTrait,
    instructions::lookup_instruction_set,
    models::{Context, FakeDisassembly, ProcessorMode, Registers, PSR},
};

#[derive(PartialEq)]
pub enum ArmKind {
    ARM9,
    ARM7,
}

impl From<ArmKind> for bool {
    fn from(kind: ArmKind) -> bool {
        match kind {
            ArmKind::ARM9 => true,
            ArmKind::ARM7 => false,
        }
    }
}

pub struct ArmBool;
impl ArmBool {
    pub const ARM9: bool = true;
    pub const ARM7: bool = false;
}

pub struct Arm<Bus: BusTrait> {
    _phantom: std::marker::PhantomData<Bus>,

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

    // TODO: do this better
    // it's 2am i cannot be bothered
    // arm9 exclusives
    pub cp15: CP15,
    // arm7 exlusives
    pub wram7: Vec<u8>, // 64kb

    // emulator variables
    pub pc_changed: bool,
}

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

    fn handle_irq(&mut self);

    fn cp15(&self) -> &CP15;
    fn cp15_mut(&mut self) -> &mut CP15;

    fn read_byte(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u8;
    fn read_halfword(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u16;
    fn read_word(&self, bus: &mut Bus, shared: &mut Shared, addr: u32) -> u32;

    fn write_byte(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u8);
    fn write_halfword(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u16);
    fn write_word(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, value: u32);
}

impl<Bus: BusTrait> Default for Arm<Bus> {
    fn default() -> Arm<Bus> {
        // TODO: in the future, the stack pointer MIGHT be set by the BIOS?
        let (sp, irq_sp, svc_sp) = if Bus::KIND == ArmKind::ARM9 {
            (0x00803EC0, 0x00803FA0, 0x00803FC0)
        } else {
            (0x0380FF00, 0x0380FFB0, 0x0380FFDC)
        };

        Arm::<Bus> {
            _phantom: std::marker::PhantomData,

            r: Registers::new_with_sp(sp),
            r_fiq: [0, 0, 0, 0, 0, 0, 0, 0],
            r_irq: [irq_sp, 0, 0],
            r_svc: [svc_sp, 0, 0],
            r_abt: [0, 0, 0],
            r_und: [0, 0, 0],
            cpsr: PSR::default(),

            cp15: CP15::default(),
            wram7: vec![0; 1024 * 64],

            pc_changed: true,
        }
    }
}

impl<Bus: BusTrait> Arm<Bus> {
    pub fn clock(&mut self, bus: &mut Bus, shared: &mut Shared) -> u32 {
        let is_thumb = self.cpsr.get_thumb();
        let inst = if is_thumb {
            self.read_halfword(bus, shared, self.r[15]) as u32
        } else {
            self.read_word(bus, shared, self.r[15])
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
            ArmKind::ARM9 => {
                lookup_instruction_set::<true>(&mut Context::new(
                    inst,
                    self,
                    bus,
                    shared,
                    &mut FakeDisassembly,
                    &mut logger::Logger(logger::LogSource::Arm9(inst)),
                )) + 2
            }
            ArmKind::ARM7 => lookup_instruction_set::<false>(&mut Context::new(
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
        }

        self.pc_changed = false;

        if !self.cpsr().get_irq_interrupt() && bus.is_requesting_interrupt() {
            self.handle_irq();
        }

        cycles
    }
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
                    ArmKind::ARM9 => logger::LogSource::Arm9(0),
                    ArmKind::ARM7 => logger::LogSource::Arm7(0),
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
                    ArmKind::ARM9 => logger::LogSource::Arm9(0),
                    ArmKind::ARM7 => logger::LogSource::Arm7(0),
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

    fn handle_irq(&mut self) {
        let instruction_width = if self.cpsr().get_thumb() { 2 } else { 4 };
        self.set_mode_r(ProcessorMode::IRQ, 1, self.r[15] + instruction_width + 4);
        self.switch_mode::<false>(ProcessorMode::IRQ, true);
        self.cpsr_mut().set_thumb(false);
        self.cpsr_mut().set_irq_interrupt(true);

        if Bus::KIND == ArmKind::ARM9 {
            self.set_r(15, 0xFFFF0018);
        } else {
            self.set_r(15, 0x00000018);
        }
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

impl<Bus: BusTrait> Arm<Bus> {
    pub fn read_bulk(&self, bus: &mut Bus, shared: &mut Shared, addr: u32, len: u32) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..len {
            bytes.push(self.read_byte(bus, shared, addr + i));
        }
        bytes
    }

    pub fn write_bulk(&mut self, bus: &mut Bus, shared: &mut Shared, addr: u32, data: Vec<u8>) {
        (0..data.len()).for_each(|i| {
            self.write_byte(bus, shared, addr + i as u32, data[i]);
        });
    }

    #[inline(always)]
    fn read_slice<const T: usize>(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        orig_addr: u32,
    ) -> [u8; T] {
        let addr = orig_addr as usize / T * T;
        let mut bytes = [0; T];

        let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
            self.cp15.data_tcm_base as usize,
            self.cp15.data_tcm_size as usize,
            self.cp15.inst_tcm_base as usize,
            self.cp15.inst_tcm_size as usize,
        );
        let (data_tcm_end, inst_tcm_end) =
            (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);
        match Bus::KIND {
            ArmKind::ARM9 => {
                if !self.cp15.control_register.get_instruction_tcm_load_mode()
                    && addr >= inst_tcm_base
                    && addr < inst_tcm_end
                {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    bytes.copy_from_slice(&self.cp15.inst_tcm[addr..addr + T]);
                    return bytes;
                }
                if !self.cp15.control_register.get_data_tcm_load_mode()
                    && addr >= data_tcm_base
                    && addr < data_tcm_end
                {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    bytes.copy_from_slice(&self.cp15.data_tcm[addr..addr + T]);
                    return bytes;
                }

                bus.read_slice::<T>(shared, orig_addr)
            }
            ArmKind::ARM7 => match addr {
                0x03800000..=0x03FFFFFF => {
                    let addr = (addr - 0x03800000) % 0x10000;
                    bytes.copy_from_slice(&self.wram7[addr..addr + T]);
                    bytes
                }
                _ => bus.read_slice::<T>(shared, orig_addr),
            },
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        orig_addr: u32,
        value: [u8; T],
    ) {
        let addr = orig_addr as usize / T * T;

        match Bus::KIND {
            ArmKind::ARM9 => {
                let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
                    self.cp15.data_tcm_base as usize,
                    self.cp15.data_tcm_size as usize,
                    self.cp15.inst_tcm_base as usize,
                    self.cp15.inst_tcm_size as usize,
                );
                let (data_tcm_end, inst_tcm_end) =
                    (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);

                if addr >= inst_tcm_base && addr < inst_tcm_end {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    self.cp15.inst_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }
                if addr >= data_tcm_base && addr < data_tcm_end {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    self.cp15.data_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }

                bus.write_slice::<T>(shared, orig_addr, value)
            }
            ArmKind::ARM7 => match addr {
                0x03800000..=0x03FFFFFF => {
                    let addr = (addr - 0x03800000) % 0x10000;
                    self.wram7[addr..addr + T].copy_from_slice(&value);
                }
                _ => bus.write_slice::<T>(shared, orig_addr, value),
            },
        };
    }
}

pub struct FakeArm {
    r: Registers,
    cpsr: PSR,
    cp15: CP15,
}

impl FakeArm {
    pub fn new(r15: u32) -> FakeArm {
        FakeArm {
            r: Registers::new_with_pc(r15),
            cpsr: PSR::default(),
            cp15: CP15::default(),
        }
    }
}

impl<Bus: BusTrait> ArmTrait<Bus> for FakeArm {
    fn r(&self) -> &Registers {
        &self.r
    }

    fn set_r(&mut self, r: u8, value: u32) {
        self.r[r] = value;
    }

    fn er(&self, r: u8) -> u32 {
        match r {
            15 => self.r[15] + 8,
            _ => self.r[r],
        }
    }

    fn ert(&self, r: u8) -> u32 {
        match r {
            15 => self.r[15] + 4,
            _ => self.r[r],
        }
    }

    fn eru(&self, r: u8) -> u32 {
        self.r[r]
    }

    fn set_mode_r(&mut self, _mode: ProcessorMode, r: u8, value: u32) {
        self.r[r] = value;
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

    fn handle_irq(&mut self) {}

    fn cp15(&self) -> &CP15 {
        &self.cp15
    }

    fn cp15_mut(&mut self) -> &mut CP15 {
        &mut self.cp15
    }

    fn read_byte(&self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32) -> u8 {
        0
    }
    fn read_halfword(&self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32) -> u16 {
        0
    }
    fn read_word(&self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32) -> u32 {
        0
    }
    fn write_byte(&mut self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32, _value: u8) {}
    fn write_halfword(&mut self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32, _value: u16) {}
    fn write_word(&mut self, _bus: &mut Bus, _shared: &mut Shared, _addr: u32, _value: u32) {}
}
