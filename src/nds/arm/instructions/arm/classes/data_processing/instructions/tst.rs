use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// TST
pub fn tst(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("TST");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);

    let first_source_register = arm.er(inst.first_source_register);
    let alu_out = first_source_register & inst.second_source_operand;
    arm.cpsr_mut().set_negative(alu_out.get_bit(31));
    arm.cpsr_mut().set_zero(alu_out == 0);
    arm.cpsr_mut().set_carry(inst.carry_out);
}
