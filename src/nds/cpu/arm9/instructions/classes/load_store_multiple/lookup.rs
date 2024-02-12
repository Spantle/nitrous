use crate::nds::{
    cpu::arm9::models::{Context, ContextTrait, DisassemblyTrait, Instruction},
    logger::LoggerTrait,
};

use super::{instructions, LoadStoreMultipleInstruction};

#[inline(always)]
pub fn lookup<Ctx: ContextTrait>(inst_set: u16, ctx: &mut Context<Instruction, Ctx>) -> u32 {
    let ctx = Context::<_, Ctx> {
        inst: LoadStoreMultipleInstruction::new(inst_set, ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,
        dis: ctx.dis,
        logger: ctx.logger,
    };

    let s = inst_set >> 2 & 1 == 1; // S
    let is_incremented = inst_set >> 1 & 1 == 1; // W
    let is_load = inst_set & 1 == 1; // L
    let has_15 = ctx.inst.register_list >> 15 & 1 == 1;

    if is_load {
        ctx.dis.set_inst("LDM");

        if s {
            ctx.dis.push_str_end_arg("^", None);

            if has_15 {
                return instructions::ldm_3(ctx);
            } else if !is_incremented {
                return instructions::ldm_2(ctx);
            }
        } else {
            return instructions::ldm_1(ctx);
        }
    } else {
        // TODO
    }

    ctx.logger.log_warn("unknown load/store multiple inst");
    1
}
