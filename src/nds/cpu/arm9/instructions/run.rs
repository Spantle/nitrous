use crate::nds::{
    cpu::{arm9::Arm9, bus::Bus},
    logger,
};

use super::{classes::lookup_instruction_class, conditions::calculate_cond, models::Instruction};

#[cfg(not(feature = "epic"))]
pub fn run_instruction_set(inst: Instruction, arm9: &mut Arm9, bus: &mut Bus) -> u32 {
    let inst_set = (inst.bits() >> 20 & 0b111111111111) as u16;
    let cond_result = calculate_cond(arm9, inst_set);
    if !cond_result {
        logger::debug("condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(inst_set, inst, arm9, bus)
}

#[cfg(feature = "epic")]
pub fn run_instruction_set<const INST_SET: u16>(
    inst: Instruction,
    arm9: &mut Arm9,
    bus: &mut Bus,
) -> u32 {
    let cond_result = calculate_cond(arm9, INST_SET);
    if !cond_result {
        logger::debug("condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(INST_SET, inst, arm9, bus)
}
