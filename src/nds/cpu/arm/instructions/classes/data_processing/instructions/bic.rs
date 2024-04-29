use crate::nds::cpu::arm::{
    arm::ArmTrait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// BIC, BICS
pub fn bic<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("BIC");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    let result = arm.er(inst.first_source_register) & !inst.second_source_operand;
    arm.r()[inst.destination_register] = result;

    if S {
        if inst.destination_register == 15 {
            arm.set_cpsr(arm.get_spsr());
        } else {
            arm.cpsr().set_negative(result.get_bit(31));
            arm.cpsr().set_zero(result == 0);
            arm.cpsr().set_carry(inst.carry_out);
        }
    }
}
