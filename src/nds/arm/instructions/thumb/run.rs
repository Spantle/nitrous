use crate::nds::arm::{
    models::{Context, ContextTrait},
    ArmKind,
};

use super::{classes::lookup_instruction_class, Instruction};

// not sure why this exists

#[cfg(not(feature = "epic"))]
pub fn run_instruction<const ARM_BOOL: bool>(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    lookup_instruction_class(ArmKind::from_bool(ARM_BOOL), inst_set, ctx)
}

#[cfg(feature = "epic")]
pub fn run_instruction<const ARM_BOOL: bool, const INST_SET: u16>(
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    lookup_instruction_class(ArmKind::from_bool(ARM_BOOL), INST_SET, ctx)
}
