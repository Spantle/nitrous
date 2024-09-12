use crate::nds::arm::{
    instructions::arm::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// SBC, SBCS
pub fn sbc<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("SBC");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    let first_source_register = arm.er(inst.first_source_register);
    let c_flag = !arm.cpsr().get_carry() as u32;
    if S {
        let (result, borrow1) = first_source_register.overflowing_sub(inst.second_source_operand);
        let (result, borrow2) = result.overflowing_sub(c_flag);
        let borrow = borrow1 || borrow2;
        arm.set_r(inst.destination_register, result);

        if inst.destination_register == 15 {
            arm.set_cpsr(arm.get_spsr());
        } else {
            arm.cpsr_mut().set_negative(result.get_bit(31));
            arm.cpsr_mut().set_zero(result == 0);
            arm.cpsr_mut().set_carry(!borrow);

            let (result, overflow1) =
                (first_source_register as i32).overflowing_sub(inst.second_source_operand as i32);
            let (_, overflow2) = result.overflowing_sub(c_flag as i32);
            arm.cpsr_mut().set_overflow(overflow1 || overflow2);
        }
    } else {
        arm.set_r(
            inst.destination_register,
            first_source_register
                .wrapping_sub(inst.second_source_operand)
                .wrapping_sub(c_flag),
        );
    }
}
