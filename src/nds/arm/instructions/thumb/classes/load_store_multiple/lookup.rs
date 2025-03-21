use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let register_list = ctx.inst.get_halfword(0, 7);

    ctx.dis.push_str_end_arg("", Some("{"));
    let mut prefix = "";
    for i in 0..=7 {
        if register_list.get_bit(i as u16) {
            ctx.dis.push_reg_end_arg(i, Some(prefix));
            prefix = ",";
        }
    }
    ctx.dis.push_str_end_arg("", Some("}"));

    if (inst_set >> 5) & 0b1 == 0 {
        // STMIA
        instructions::stmia(ctx);
    } else {
        // LDMIA
        instructions::ldmia(ctx);
    }

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_push_pop(
    arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let pop = ((inst_set >> 5) & 0b1) == 1;
    let register_list = ctx.inst.get_halfword(0, 7);
    let r = ctx.inst.get_bit(8);

    ctx.dis.push_str_end_arg("", Some("{"));
    let mut prefix = "";
    for i in 0..=7 {
        if register_list.get_bit(i as u16) {
            ctx.dis.push_reg_end_arg(i, Some(prefix));
            prefix = ",";
        }
    }

    if pop {
        if r {
            ctx.dis.push_str_end_arg("PC", Some(prefix));
        }
        instructions::pop(arm_bool, ctx);
    } else {
        if r {
            ctx.dis.push_str_end_arg("LR", Some(prefix));
        }
        instructions::push(ctx);
    }

    ctx.dis.push_str_end_arg("", Some("}"));

    1 // TODO: this is wrong
}
