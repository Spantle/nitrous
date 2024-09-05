use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// BLX (2), BX
pub fn bx<const L: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    if L {
        ctx.dis.set_inst("BLX");
    } else {
        ctx.dis.set_inst("BX");
    }

    if L {
        ctx.arm.set_r(14, (ctx.arm.r()[15] + 2) | 1);
    }

    let rm = ctx.inst.get_byte(3, 5);
    let h2 = ctx.inst.get_bit(6);
    let rm = ctx.inst.get_rh(h2, rm);
    ctx.dis.push_reg_arg(rm, None);

    let rm = ctx.arm.ert(rm);
    ctx.arm.cpsr_mut().set_thumb(rm.get_bit(0));
    ctx.arm.set_r(15, rm.get_bits(1, 31) << 1);

    1 // TODO: this is wrong
}
