use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn ascm_immediate_lookup<Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let opcode = inst_set >> 4 & 0b111;
    match opcode {
        0b000 => {
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
