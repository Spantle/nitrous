use crate::nds::cpu::arm::{
    arm::ArmTrait,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

// MRS
#[inline(always)]
pub fn mrs(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MRS");

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);

    let r = inst_set >> 2 & 1 == 1;
    let rd = inst.get_byte(12, 15);
    ctx.dis.push_reg_arg(rd, None);

    if r {
        ctx.dis.push_str_end_arg("SPSR", None);
        arm.r()[rd] = arm.get_spsr().value();
    } else {
        ctx.dis.push_str_end_arg("CPSR", None);
        arm.r()[rd] = arm.cpsr().value();
    }

    1 // TODO: this is wrong
}
