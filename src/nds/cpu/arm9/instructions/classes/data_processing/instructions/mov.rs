use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// MOV, MOVS
pub fn mov<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("MOV");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);
    let result = inst.second_source_operand;
    arm9.r()[inst.destination_register] = result;

    if S {
        if inst.destination_register == 15 {
            arm9.set_cpsr(arm9.get_spsr());
        } else {
            arm9.cpsr().set_negative(result.get_bit(31));
            arm9.cpsr().set_zero(result == 0);
            arm9.cpsr().set_carry(inst.carry_out);
        }
    }
}
