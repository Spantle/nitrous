use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let opcode = (inst_set >> 5) & 0b11;
    match opcode {
        0b01 => {
            // LSR (1)
            instructions::lsr_1(ctx)
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown data processing lookup opcode {:03b}",
                opcode
            ));
            return 10000;
        }
    };

    1 // TODO: this is wrong
}

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
