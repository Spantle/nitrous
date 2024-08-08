use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// MVN, MVNS
pub fn mvn<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("MVN");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    let result = !inst.second_source_operand;
    arm.set_r(inst.destination_register, result);

    if S {
        if inst.destination_register == 15 {
            arm.set_cpsr(arm.get_spsr());
        } else {
            arm.cpsr_mut().set_negative(result.get_bit(31));
            arm.cpsr_mut().set_zero(result == 0);
            arm.cpsr_mut().set_carry(inst.carry_out);
        }
    }
}