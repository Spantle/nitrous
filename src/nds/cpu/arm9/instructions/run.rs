use crate::nds::{cpu::arm9::models::Context, logger};

use super::{classes::lookup_instruction_class, conditions::calculate_cond};

#[cfg(not(feature = "epic"))]
pub fn run_instruction_set(inst_set: u16, ctx: Context) -> u32 {
    let cond_result = calculate_cond(ctx.arm9, inst_set);
    if !cond_result {
        logger::debug(logger::LogSource::Arm9, "condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(inst_set, ctx)
}

#[cfg(feature = "epic")]
pub fn run_instruction_set<const INST_SET: u16>(ctx: Context) -> u32 {
    let cond_result = calculate_cond(ctx.arm9, INST_SET);
    if !cond_result {
        logger::debug(logger::LogSource::Arm9, "condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(INST_SET, ctx)
}
