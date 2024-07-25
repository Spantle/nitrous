use crate::nds::cpu::arm::{
    arm::ArmTrait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// CMN
pub fn cmn(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("CMN");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);

    let first_source_register = arm.er(inst.first_source_register);
    let (alu_out, carry) = first_source_register.overflowing_add(inst.second_source_operand);
    arm.cpsr_mut().set_negative(alu_out.get_bit(31));
    arm.cpsr_mut().set_zero(alu_out == 0);
    arm.cpsr_mut().set_carry(carry);
    arm.cpsr_mut().set_overflow(
        (first_source_register as i32)
            .overflowing_add(inst.second_source_operand as i32)
            .1,
    );
}
