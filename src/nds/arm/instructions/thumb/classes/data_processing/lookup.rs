use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn ascm_immediate_lookup(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let opcode = (inst_set >> 5) & 0b11;
    match opcode {
        0b00 => {
            // MOV
            instructions::mov(ctx)
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown ascm immediate lookup opcode {:03b}",
                opcode
            ));
        }
    };

    1 // TODO: this is wrong
}
