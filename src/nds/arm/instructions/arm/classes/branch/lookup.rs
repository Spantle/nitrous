use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait},
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let l = ((inst_set >> 4) & 1) != 0; // L bit

    if l {
        instructions::b::<true, false>(ctx)
    } else {
        instructions::b::<false, false>(ctx)
    }
}
