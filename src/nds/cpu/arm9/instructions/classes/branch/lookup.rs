use crate::nds::cpu::arm9::models::{Context, ContextTrait, DisassemblyTrait, Instruction};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let l = ((inst_set >> 4) & 1) != 0; // L bit

    if l {
        ctx.dis.set_inst("BL");
        instructions::b::<true>(ctx)
    } else {
        ctx.dis.set_inst("B");
        instructions::b::<false>(ctx)
    }
}
