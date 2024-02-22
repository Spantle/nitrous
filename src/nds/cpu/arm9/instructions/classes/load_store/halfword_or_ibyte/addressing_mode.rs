// this was surprisingly easy
// almost too easy...
// either that or i did all the hard ones first

use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

#[inline(always)]
pub fn parse_immediate(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let offset_8 = (ctx.inst.get_word(8, 11) << 4) | ctx.inst.get_word(0, 3);
    ctx.dis.push_word_end_arg(offset_8, Some(", "));

    offset_8
}

#[inline(always)]
pub fn parse_register(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let rm = ctx.inst.get_byte(0, 3);
    ctx.dis.push_reg_end_arg(rm, Some(", "));

    ctx.arm9.eru(rm)
}
