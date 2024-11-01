use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait},
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    // bit 22
    if inst_set >> 2 & 1 == 0 {
        instructions::swp(ctx)
    } else {
        instructions::swpb(ctx)
    }
}
