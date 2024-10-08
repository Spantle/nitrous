use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    if ctx.inst.get_bit(4) {
        // Coprocessor register transfers
        if inst_set & 1 == 0 {
            instructions::mcr(ctx)
        } else {
            instructions::mrc(ctx)
        }
    } else {
        // Coprocessor data processing
        ctx.logger
            .log_error("coprocessor data processing instructions not implemented");
        1
    }
}
