use crate::nds::arm::models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction};

use super::{instructions, LoadStoreMultipleInstruction};

#[inline(always)]
pub fn lookup<Ctx: ContextTrait>(inst_set: u16, ctx: &mut Context<Instruction, Ctx>) -> u32 {
    let ctx = Context::new(
        LoadStoreMultipleInstruction::new(inst_set, ctx),
        ctx.arm,
        ctx.bus,
        ctx.shared,
        ctx.dis,
        ctx.logger,
    );

    let s = inst_set >> 2 & 1 == 1; // S
    let is_load = inst_set & 1 == 1; // L
    let has_15 = ctx.inst.register_list.get_bit(15);

    if is_load {
        ctx.dis.set_inst("LDM");

        if s {
            ctx.dis.push_str_end_arg("^", None);

            if has_15 {
                instructions::ldm_3(inst_set, ctx)
            } else {
                // assumes W bit is 0
                instructions::ldm_2(inst_set, ctx)
            }
        } else {
            instructions::ldm_1(inst_set, ctx)
        }
    } else {
        ctx.dis.set_inst("STM");

        if s {
            // assumes W bit is 0
            instructions::stm_2(inst_set, ctx)
        } else {
            instructions::stm_1(inst_set, ctx)
        }
    }
}
