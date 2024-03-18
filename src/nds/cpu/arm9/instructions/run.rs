use crate::nds::{
    cpu::arm9::models::{Context, ContextTrait, Instruction},
    logger::LoggerTrait,
};

use super::{classes::lookup_instruction_class, conditions::calculate_cond};

#[cfg(not(feature = "epic"))]
pub fn run_instruction_set(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    use crate::nds::cpu::arm9::models::DisassemblyTrait;

    let cond_result = calculate_cond(inst_set, ctx);
    if !ctx.dis.is_real() && !cond_result {
        ctx.logger.log_debug(format!(
            "condition failed {:#06X} ({:016b})",
            inst_set, inst_set
        ));
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(inst_set, ctx)
}

#[cfg(feature = "epic")]
pub fn run_instruction_set<const INST_SET: u16>(
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let cond_result = calculate_cond(INST_SET, ctx);
    if !cond_result {
        ctx.logger.log_debug(format!(
            "condition failed {:#06X} ({:016b})",
            inst_set, inst_set
        ));
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(INST_SET, ctx)
}
