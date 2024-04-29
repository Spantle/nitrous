use crate::nds::cpu::arm::{
    arm::ArmTrait,
    models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction},
};

// BX
#[inline(always)]
pub fn bx(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("BX");

    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    let rm = inst.get_byte(0, 3);
    ctx.dis.push_reg_arg(rm, None);
    let rm = arm.er(inst.get_byte(0, 3));

    let thumb = rm.get_bit(0);
    arm.cpsr().set_thumb(thumb);
    arm.r()[15] = rm & 0xFFFFFFFE;

    1 // TODO: this is wrong
}
