use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
        ArmKind,
    },
    logger::LoggerTrait,
};

#[inline(always)]
pub fn lookup_instruction_class(
    arm_kind: ArmKind,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 7) & 0b111;

    match class {
        _ => {
            ctx.logger
                .log_warn(format!("unknown instruction class {:03b}", class));
            1
        }
    }
}
