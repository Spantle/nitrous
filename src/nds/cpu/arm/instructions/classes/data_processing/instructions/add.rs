use crate::nds::cpu::arm::{
    arm::ArmTrait,
    instructions::classes::data_processing::DataProcessingInstruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
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
        arm.r()[inst.destination_register] = result;

        if inst.destination_register == 15 {
            arm.set_cpsr(arm.get_spsr());
        } else {
            arm.cpsr().set_negative(result.get_bit(31));
            arm.cpsr().set_zero(result == 0);
            arm.cpsr().set_carry(carry);
            arm.cpsr().set_overflow(
                (first_source_register as i32)
                    .overflowing_add(inst.second_source_operand as i32)
                    .1,
            );
        }
    } else {
        arm.r()[inst.destination_register] = arm
            .er(inst.first_source_register)
            .wrapping_add(inst.second_source_operand);
    }
}
