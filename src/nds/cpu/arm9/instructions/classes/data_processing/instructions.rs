use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        models::{Context, DisassemblyTrait},
    },
    bus::BusTrait,
};

use super::DataProcessingInstruction;

// MOV, MOVS
pub fn mov<const S: bool>(
    ctx: &mut Context<
        DataProcessingInstruction,
        impl Arm9Trait,
        impl BusTrait,
        impl DisassemblyTrait,
    >,
) {
    ctx.dis.push_reg_arg(ctx.inst.destination_register);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);
    let result = inst.second_source_operand;
    arm9.r()[inst.destination_register] = result;

    if S {
        if inst.destination_register == 15 {
            arm9.set_cpsr(arm9.get_spsr());
        } else {
            arm9.cpsr().set_negative(result & (1 << 31) != 0);
            arm9.cpsr().set_zero(result == 0);
            arm9.cpsr().set_carry(inst.carry_out);
        }
    }
}

// ADD, ADDS
pub fn add<const S: bool>(
    ctx: &mut Context<
        DataProcessingInstruction,
        impl Arm9Trait,
        impl BusTrait,
        impl DisassemblyTrait,
    >,
) {
    ctx.dis.push_reg_arg(ctx.inst.destination_register);
    ctx.dis.push_reg_arg(ctx.inst.first_source_register);

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);
    if S {
        let first_source_register = arm9.er(inst.first_source_register);
        let (result, overflow) = first_source_register.overflowing_add(inst.second_source_operand);
        arm9.r()[inst.destination_register] = result;

        if inst.destination_register == 15 {
            arm9.set_cpsr(arm9.get_spsr());
        } else {
            arm9.cpsr().set_negative(result & (1 << 31) != 0);
            arm9.cpsr().set_zero(result == 0);
            arm9.cpsr().set_carry(overflow);
            arm9.cpsr().set_overflow(
                (first_source_register & (1 << 31) == 0)
                    && (inst.second_source_operand & (1 << 31) == 0)
                    && (result & (1 << 31) != 0),
            );
        }
    } else {
        arm9.r()[inst.destination_register] = arm9
            .er(inst.first_source_register)
            .wrapping_add(inst.second_source_operand);
    }
}
