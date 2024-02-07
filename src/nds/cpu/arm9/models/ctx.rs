use crate::nds::cpu::{arm9::arm9::Arm9Trait, bus::BusTrait};

use super::disassembly::DisassemblyTrait;

pub struct Context<'a, Inst, Cpu: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait> {
    pub inst: Inst,
    pub arm9: &'a mut Cpu,
    pub bus: &'a mut Bus,

    pub dis: &'a mut Dis,
}
