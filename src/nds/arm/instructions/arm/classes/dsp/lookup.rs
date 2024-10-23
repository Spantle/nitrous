use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup_multiplies(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    // bits 21-22
    let opcode = inst_set >> 1 & 0b11;

    match opcode {
        0b00 => instructions::smla(ctx),
        0b11 => instructions::smul(ctx),
        0b01 | 0b10 => {
            ctx.logger.log_error(format!(
                "DSP multiplies opcode {:02b} not implemented",
                opcode
            ));
            100
        }
        _ => unreachable!(),
    }
}
