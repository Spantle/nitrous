use crate::nds::{
    arm::{
        arm::ArmTrait,
        models::{Context, ContextTrait},
    },
    logger::{self, LoggerTrait},
};

use super::{arm, thumb};

pub fn lookup_instruction_set<const ARM_BOOL: bool>(
    ctx: &mut Context<u32, impl ContextTrait>,
) -> u32 {
    let thumb = ctx.arm.cpsr().get_thumb();
    if thumb {
        let inst = ctx.inst as u16;
        if ARM_BOOL {
            ctx.logger.set_source(logger::LogSource::Arm9T(inst));
        } else {
            ctx.logger.set_source(logger::LogSource::Arm7T(inst));
        }

        thumb::lookup_instruction::<ARM_BOOL>(&mut Context::new(
            thumb::Instruction::from(inst),
            ctx.arm,
            ctx.bus,
            ctx.shared,
            ctx.dis,
            ctx.logger,
        ))
    } else {
        arm::lookup_instruction::<ARM_BOOL>(&mut Context::new(
            arm::Instruction::from(ctx.inst),
            ctx.arm,
            ctx.bus,
            ctx.shared,
            ctx.dis,
            ctx.logger,
        ))
    }
}
