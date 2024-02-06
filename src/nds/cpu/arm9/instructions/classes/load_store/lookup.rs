use crate::nds::{cpu::arm9::models::Context, logger};

use super::{instructions, LoadStoreInstruction};

#[inline(always)]
pub fn lookup<const IS_REGISTER: bool>(inst_set: u16, ctx: Context) -> u32 {
    let mut ctx = Context {
        inst: LoadStoreInstruction::new::<IS_REGISTER>(&ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,
    };
    let (arm9, inst) = (&mut ctx.arm9, &ctx.inst);

    let post_indexing = inst_set >> 4 & 1 == 0; // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    let is_add = inst_set >> 3 & 1 == 1; // U
    let is_unsigned_byte = inst_set >> 2 & 1 == 1; // B
    let w = inst_set >> 1 & 1 == 1; // W
    let is_load = inst_set & 1 == 1; // L

    let address = if post_indexing {
        let address = arm9.er(inst.first_source_register);
        if is_add {
            arm9.r[inst.first_source_register] += inst.addressing_mode;
        } else {
            arm9.r[inst.first_source_register] -= inst.addressing_mode;
        }
        address
    } else if is_add {
        arm9.er(inst.first_source_register) + inst.addressing_mode
    } else {
        arm9.er(inst.first_source_register) - inst.addressing_mode
    };

    // logger::debug(
    //     logger::LogSource::Arm9,
    //     format!(
    //         "load/store address: {:#010X} ({}) ({}) {:#010X}",
    //         address,
    //         inst.first_source_register,
    //         arm9.er(inst.first_source_register),
    //         inst.addressing_mode
    //     ),
    // );
    // logger::debug(
    //     logger::LogSource::Arm9,
    //     format!(
    //         "addressing mode: {} {} {} {} {} {}",
    //         IS_REGISTER, post_indexing, is_add, is_unsigned_byte, w, is_load
    //     ),
    // );

    if w {
        arm9.r[inst.first_source_register] = address;
    };

    // there's also some unpredictability if it's "Scaled register pre-indexed" and Rn and Rm are the same
    // i'm sure it's fine
    if !is_unsigned_byte {
        if is_load {
            return instructions::ldr(ctx, address);
        } else {
            return instructions::str(ctx, address);
        }
    } else {
        if is_load {
            // return instructions::ldrb(ctx, address);
        } else {
            return instructions::strb(ctx, address);
        }
    }

    logger::warn(
        logger::LogSource::Arm9,
        format!(
            "unknown load/store inst {} {} {} {} {} {}",
            IS_REGISTER, post_indexing, is_add, is_unsigned_byte, w, is_load
        ),
    );
    1
}
