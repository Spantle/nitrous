use crate::nds::{bus::BusTrait, cp15::CP15, dma::Dma, shared::Shared};

use super::{
    models::{ProcessorMode, Psr, Registers, StackTrace},
    ArmTrait,
};

pub struct FakeArm {
    r: Registers,
    cpsr: Psr,
    stacktrace: StackTrace,
    cp15: CP15,
}

impl FakeArm {
    pub fn new(r15: u32) -> FakeArm {
        FakeArm {
            r: Registers::new_with_pc(r15),
            cpsr: Psr::default(),
            stacktrace: StackTrace::default(),
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

    fn cpsr(&self) -> &Psr {
        &self.cpsr
    }

    fn cpsr_mut(&mut self) -> &mut Psr {
        &mut self.cpsr
    }

    fn set_cpsr(&mut self, _psr: Psr) {}

    fn get_spsr(&self) -> Psr {
        Psr::default()
    }

    fn set_spsr(&mut self, _psr: Psr) {}

    fn switch_mode<const RETURN_TO_DEFAULT: bool>(
        &mut self,
        _mode: ProcessorMode,
        _copy_cpsr_to_spsr: bool,
    ) {
    }

    fn halt(&mut self) {}

    fn stacktrace_mut(&mut self) -> &mut StackTrace {
        &mut self.stacktrace
    }

    fn cp15(&self) -> &CP15 {
        &self.cp15
    }

    fn cp15_mut(&mut self) -> &mut CP15 {
        &mut self.cp15
    }

    fn read_byte(&self, _bus: &mut Bus, _shared: &mut Shared, _dma: &mut Dma, _addr: u32) -> u8 {
        0
    }
    fn read_halfword(
        &self,
        _bus: &mut Bus,
        _shared: &mut Shared,
        _dma: &mut Dma,
        _addr: u32,
    ) -> u16 {
        0
    }
    fn read_word(&self, _bus: &mut Bus, _shared: &mut Shared, _dma: &mut Dma, _addr: u32) -> u32 {
        0
    }
    fn write_byte(
        &mut self,
        _bus: &mut Bus,
        _shared: &mut Shared,
        _dma: &mut Dma,
        _addr: u32,
        _value: u8,
    ) {
    }
    fn write_halfword(
        &mut self,
        _bus: &mut Bus,
        _shared: &mut Shared,
        _dma: &mut Dma,
        _addr: u32,
        _value: u16,
    ) {
    }
    fn write_word(
        &mut self,
        _bus: &mut Bus,
        _shared: &mut Shared,
        _dma: &mut Dma,
        _addr: u32,
        _value: u32,
    ) {
    }
}
