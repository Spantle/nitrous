use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// BX, BLX (2)
#[inline(always)]
pub fn bx<const L: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    if L {
        ctx.dis.set_inst("BLX");
    } else {
        ctx.dis.set_inst("BX");
    }

    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    let rm = inst.get_byte(0, 3);
    ctx.dis.push_reg_arg(rm, None);
    let rm = arm.er(inst.get_byte(0, 3));

    let pc = arm.r()[15];
    if L {
        arm.set_r(14, pc.wrapping_add(4));
    }

    let thumb = rm.get_bit(0);
    arm.cpsr_mut().set_thumb(thumb);
    arm.set_r(15, rm & 0xFFFFFFFE);

    if L {
        arm.stacktrace_mut().link(pc);
    } else {
        arm.stacktrace_mut()
            .exchange((rm & 0xFFFFFFFE).wrapping_sub(4));
    }

    1 // TODO: this is wrong
}
