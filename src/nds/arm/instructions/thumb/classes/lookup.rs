use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
        ArmKind,
    },
    logger::LoggerTrait,
};

use super::{branch, data_processing};

#[inline(always)]
pub fn lookup_instruction_class(
    arm_kind: ArmKind,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 7) & 0b111;

    match class {
        0b001 => {
            // Add/subtract/compare/move immediate
            data_processing::ascm_immediate_lookup(inst_set, ctx)
        }
        0b111 => {
            // branching shenanigans
            branch::lookup(arm_kind, inst_set, ctx)
        }
        _ => {
            ctx.logger
                .log_warn(format!("unknown instruction class {:03b}", class));
            10000
        }
    }
}
