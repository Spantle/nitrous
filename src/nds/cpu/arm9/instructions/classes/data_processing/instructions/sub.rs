use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// SUB, SUBS
pub fn sub<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("SUB");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);
    if S {
        let first_source_register = arm9.er(inst.first_source_register);
        let (result, borrow) = first_source_register.overflowing_sub(inst.second_source_operand);
        arm9.r()[inst.destination_register] = result;

        if inst.destination_register == 15 {
            arm9.set_cpsr(arm9.get_spsr());
        } else {
            arm9.cpsr().set_negative(result.get_bit(31));
            arm9.cpsr().set_zero(result == 0);
            arm9.cpsr().set_carry(!borrow);
            arm9.cpsr().set_overflow(
                (first_source_register as i32)
                    .overflowing_add(inst.second_source_operand as i32)
                    .1,
            );
        }
    } else {
        arm9.r()[inst.destination_register] = arm9
            .er(inst.first_source_register)
            .wrapping_sub(inst.second_source_operand);
    }
}
