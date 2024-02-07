use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        models::{Context, DisassemblyTrait, Instruction},
    },
    bus::BusTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl Arm9Trait, impl BusTrait, impl DisassemblyTrait>,
) -> u32 {
    let l = ((inst_set >> 4) & 1) != 0; // L bit

    if l {
        ctx.dis.set_inst("BL");
        instructions::b::<true>(ctx)
    } else {
        ctx.dis.set_inst("B");
        instructions::b::<false>(ctx)
    }
}
