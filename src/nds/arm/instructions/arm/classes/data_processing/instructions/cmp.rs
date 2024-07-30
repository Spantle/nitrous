use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// CMP
pub fn cmp(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("CMP");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);

    let first_source_register = arm.er(inst.first_source_register);
    let (alu_out, borrow) = first_source_register.overflowing_sub(inst.second_source_operand);
    arm.cpsr_mut().set_negative(alu_out.get_bit(31));
    arm.cpsr_mut().set_zero(alu_out == 0);
    arm.cpsr_mut().set_carry(!borrow);
    arm.cpsr_mut().set_overflow(
        (first_source_register as i32)
            .overflowing_sub(inst.second_source_operand as i32)
            .1,
    );
}
