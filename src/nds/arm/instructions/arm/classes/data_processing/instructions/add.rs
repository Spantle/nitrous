use crate::nds::arm::{
    instructions::arm::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// ADD, ADDS
pub fn add<const S: bool>(ctx: &mut Context<DataProcessingInstruction, impl ContextTrait>) {
    ctx.dis.set_inst("ADD");
    ctx.dis.push_reg_arg(ctx.inst.destination_register, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(ctx.inst.first_source_register, None);

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    if S {
        let first_source_register = arm.er(inst.first_source_register);
        let (result, carry) = first_source_register.overflowing_add(inst.second_source_operand);
        arm.set_r(inst.destination_register, result);

        if inst.destination_register == 15 {
            arm.set_cpsr(arm.get_spsr());
        } else {
            arm.cpsr_mut().set_negative(result.get_bit(31));
            arm.cpsr_mut().set_zero(result == 0);
            arm.cpsr_mut().set_carry(carry);
            arm.cpsr_mut().set_overflow(
                (first_source_register as i32)
                    .overflowing_add(inst.second_source_operand as i32)
                    .1,
            );
        }
    } else {
        arm.set_r(
            inst.destination_register,
            arm.er(inst.first_source_register)
                .wrapping_add(inst.second_source_operand),
        );
    }
}
