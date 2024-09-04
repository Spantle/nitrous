use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
};

use super::{classes::lookup_instruction_class, conditions::calculate_cond};

#[cfg(not(feature = "epic"))]
pub fn run_instruction<const ARM_BOOL: bool>(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let cond = ((inst_set >> 8) & 0b1111) as u8;
    let cond_result = calculate_cond(cond, ctx);
    if !ctx.dis.is_real() && !cond_result {
        // ctx.logger.log_debug(format!(
        //     "condition failed {:#06X} ({:016b})",
        //     inst_set, inst_set
        // ));
        return 1;
    }

    // also runs it ignore the name
    lookup_instruction_class(ARM_BOOL, inst_set, ctx)
}

#[cfg(feature = "epic")]
pub fn run_instruction<const ARM_BOOL: bool, const INST_SET: u16>(
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let cond = ((INST_SET >> 8) & 0b1111) as u8;
    let cond_result = calculate_cond(cond, ctx);
    if !ctx.dis.is_real() && !cond_result {
        // ctx.logger.log_debug(format!(
        //     "condition failed {:#06X} ({:016b})",
        //     INST_SET, INST_SET
        // ));
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class(ARM_BOOL, INST_SET, ctx)
}
