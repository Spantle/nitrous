use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

use super::{instructions, LoadStoreInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool, Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let mut ctx = Context::new(
        LoadStoreInstruction::new::<IS_IMMEDIATE>(inst_set, ctx),
        ctx.arm,
        ctx.bus,
        ctx.shared,
        ctx.dma,
        ctx.dis,
        ctx.logger,
    );
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    ctx.dis.push_reg_arg(inst.destination_register, None);

    let post_indexing = inst_set >> 4 & 1 == 0; // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    let is_add = inst_set >> 3 & 1 == 1; // U
    let w = inst_set >> 1 & 1 == 1; // W
    let is_load = inst_set & 1 == 1; // L
    let s = ctx.inst.is_signed; // S
    let h = ctx.inst.is_halfword; // H

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

    if w {
        ctx.dis.push_str_end_arg("!", None);
        arm.set_r(inst.first_source_register, address);
    };

    match (s, h, is_load) {
        (true, true, true) => {
            ctx.dis.set_inst("LDRSH");
            instructions::ldrsh(&mut ctx, address);
        }
        (true, false, true) => {
            ctx.dis.set_inst("LDRSB");
            instructions::ldrsb(&mut ctx, address);
        }
        (false, true, true) => {
            ctx.dis.set_inst("LDRH");
            instructions::ldrh(&mut ctx, address);
        }
        (false, true, false) => {
            ctx.dis.set_inst("STRH");
            instructions::strh(&mut ctx, address);
        }
        _ => unreachable!("If you see this then the software you're running sucks"), // might actually be reachable
    };

    1
}
