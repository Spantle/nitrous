use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// CMP
pub fn cmp(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("CMP");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);

    let first_source_register = arm9.er(inst.first_source_register);
    let (alu_out, borrow) = first_source_register.overflowing_sub(inst.second_source_operand);
    arm9.cpsr().set_negative(alu_out.get_bit(31));
    arm9.cpsr().set_zero(alu_out == 0);
    arm9.cpsr().set_carry(!borrow);
    arm9.cpsr().set_overflow(
        (first_source_register as i32)
            .overflowing_add_unsigned(inst.second_source_operand)
            .1,
    );
}
