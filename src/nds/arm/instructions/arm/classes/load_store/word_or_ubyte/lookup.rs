use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

use super::{instructions, LoadStoreInstruction};

#[inline(always)]
pub fn lookup<const IS_REGISTER: bool, Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let mut ctx = Context::new(
        LoadStoreInstruction::new::<IS_REGISTER>(inst_set, ctx),
        ctx.arm,
        ctx.bus,
        ctx.shared,
        ctx.dis,
        ctx.logger,
    );
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    ctx.dis.push_reg_arg(inst.destination_register, None);

    let post_indexing = inst_set >> 4 & 1 == 0; // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    let is_add = inst_set >> 3 & 1 == 1; // U
    let is_unsigned_byte = inst_set >> 2 & 1 == 1; // B
    let w = inst_set >> 1 & 1 == 1; // W
    let is_load = inst_set & 1 == 1; // L

    let rn = arm.er(inst.first_source_register);

    let address = if post_indexing {
        if is_add {
            arm.set_r(
                inst.first_source_register,
                rn.wrapping_add(inst.addressing_mode),
            );
        } else {
            arm.set_r(
                inst.first_source_register,
                rn.wrapping_sub(inst.addressing_mode),
            );
        };

        rn
    } else if is_add {
        rn.wrapping_add(inst.addressing_mode)
    } else {
        rn.wrapping_sub(inst.addressing_mode)
    };

    // ctx.logger.log_debug(format_debug!(
    //     "load/store address: {:#010X} ({}) ({}) {:#010X}",
    //     address,
    //     inst.first_source_register,
    //     arm.er(inst.first_source_register),
    //     inst.addressing_mode
    // ));
    // ctx.logger.log_debug(format_debug!(
    //     "addressing mode: {} {} {} {} {} {}",
    //     IS_REGISTER, post_indexing, is_add, is_unsigned_byte, w, is_load
    // ));

    if w {
        ctx.dis.push_str_end_arg("!", None);
        arm.set_r(inst.first_source_register, address);
    };

    // there's also some unpredictability if it's "Scaled register pre-indexed" and Rn and Rm are the same
    // i'm sure it's fine
    if !is_unsigned_byte {
        if is_load {
            ctx.dis.set_inst("LDR");
            instructions::ldr(ctx, address)
        } else {
            ctx.dis.set_inst("STR");
            instructions::str(&mut ctx, address)
        }
    } else if is_load {
        ctx.dis.set_inst("LDRB");
        instructions::ldrb(&mut ctx, address)
    } else {
        ctx.dis.set_inst("STRB");
        instructions::strb(&mut ctx, address)
    }
}
