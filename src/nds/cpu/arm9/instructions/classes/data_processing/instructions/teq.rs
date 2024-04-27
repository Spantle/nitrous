use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// TEQ
pub fn teq(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("TEQ");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);

    let first_source_register = arm9.er(inst.first_source_register);
    let alu_out = first_source_register ^ inst.second_source_operand;
    arm9.cpsr().set_negative(alu_out.get_bit(31));
    arm9.cpsr().set_zero(alu_out == 0);
    arm9.cpsr().set_carry(inst.carry_out);
}
