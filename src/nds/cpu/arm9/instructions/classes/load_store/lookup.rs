use crate::nds::{
    cpu::arm9::{
        arm9::Arm9Trait,
        models::{Context, ContextTrait, DisassemblyTrait, Instruction},
    },
    logger::LoggerTrait,
};

use super::{instructions, LoadStoreInstruction};

#[inline(always)]
pub fn lookup<const IS_REGISTER: bool, Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let mut ctx = Context::<_, Ctx> {
        inst: LoadStoreInstruction::new::<IS_REGISTER>(ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,
        dis: ctx.dis,
        logger: ctx.logger,
    };
    let (arm9, inst) = (&mut ctx.arm9, &ctx.inst);
    ctx.dis.push_reg_arg(inst.destination_register, None);

    let post_indexing = inst_set >> 4 & 1 == 0; // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    let is_add = inst_set >> 3 & 1 == 1; // U
    let is_unsigned_byte = inst_set >> 2 & 1 == 1; // B
    let w = inst_set >> 1 & 1 == 1; // W
    let is_load = inst_set & 1 == 1; // L

    let address = if post_indexing {
        let address = arm9.er(inst.first_source_register);
        if is_add {
            arm9.r()[inst.first_source_register] =
                arm9.r()[inst.first_source_register].wrapping_add(inst.addressing_mode);
        } else {
            arm9.r()[inst.first_source_register] =
                arm9.r()[inst.first_source_register].wrapping_sub(inst.addressing_mode);
        }
        address
    } else if is_add {
        arm9.er(inst.first_source_register) + inst.addressing_mode
    } else {
        arm9.er(inst.first_source_register) - inst.addressing_mode
    };

    // ctx.logger.log_debug(format!(
    //     "load/store address: {:#010X} ({}) ({}) {:#010X}",
    //     address,
    //     inst.first_source_register,
    //     arm9.er(inst.first_source_register),
    //     inst.addressing_mode
    // ));
    // ctx.logger.log_debug(format!(
    //     "addressing mode: {} {} {} {} {} {}",
    //     IS_REGISTER, post_indexing, is_add, is_unsigned_byte, w, is_load
    // ));

    if w {
        ctx.dis.push_str_end_arg("!", None);
        arm9.r()[inst.first_source_register] = address;
    };

    // there's also some unpredictability if it's "Scaled register pre-indexed" and Rn and Rm are the same
    // i'm sure it's fine
    if !is_unsigned_byte {
        if is_load {
            ctx.dis.set_inst("LDR");
            return instructions::ldr(ctx, address);
        } else {
            ctx.dis.set_inst("STR");
            return instructions::str(&mut ctx, address);
        }
    } else {
        if is_load {
            ctx.dis.set_inst("LDRB");
            // return instructions::ldrb(ctx, address);
        } else {
            ctx.dis.set_inst("STRB");
            return instructions::strb(&mut ctx, address);
        }
    }

    ctx.logger.log_warn(format!(
        "unknown load/store inst {} {} {} {} {} {}",
        IS_REGISTER, post_indexing, is_add, is_unsigned_byte, w, is_load
    ));
    1
}
