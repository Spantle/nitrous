use crate::nds::{
    cpu::{
        arm9::{models::Instruction, Arm9},
        bus::Bus,
    },
    logger,
};

use super::{classes::lookup_instruction_class, conditions::calculate_cond};

#[cfg(not(feature = "epic"))]
pub fn run_instruction_set(
    inst_set: u16,
    inst: Instruction,
    arm9: &mut Arm9,
    bus: &mut Bus,
) -> u32 {
    let cond_result = calculate_cond(arm9, inst_set);
    if !cond_result {
        logger::debug(logger::LogSource::Arm9, "condition failed");
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
        logger::debug(logger::LogSource::Arm9, "condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(INST_SET, inst, arm9, bus)
}
