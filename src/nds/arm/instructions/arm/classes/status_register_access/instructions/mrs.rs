use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// MRS
#[inline(always)]
pub fn mrs(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MRS");

    let r = (inst_set >> 2) & 1 == 1;
    let rd = ctx.inst.get_byte(12, 15);
    ctx.dis.push_reg_arg(rd, None);

    if r {
        ctx.dis.push_str_end_arg("SPSR", None);
        ctx.arm.set_r(rd, ctx.arm.get_spsr().value());
    } else {
        ctx.dis.push_str_end_arg("CPSR", None);
        ctx.arm.set_r(rd, ctx.arm.cpsr().value());
    }

    1 // TODO: this is wrong
}
