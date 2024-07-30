use crate::nds::arm::{
    arm::ArmTrait,
    models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction, ProcessorMode},
};

// MSR
#[inline(always)]
pub fn msr(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MSR");

    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);

    let is_immediate = inst_set >> 5 & 1 == 1;
    let r = inst_set >> 2 & 1 == 1;
    let field_mask = inst.get_byte(16, 19);

    if r {
        ctx.dis.push_str_arg("SPSR_");
    } else {
        ctx.dis.push_str_arg("CPSR_");
    }

    if field_mask.get_bit(0) {
        ctx.dis.push_str_arg("c");
    }
    if field_mask.get_bit(1) {
        ctx.dis.push_str_arg("x");
    }
    if field_mask.get_bit(2) {
        ctx.dis.push_str_arg("s");
    }
    if field_mask.get_bit(3) {
        ctx.dis.push_str_arg("f");
    }

    let operand = if is_immediate {
        let immediate = inst.get_word(0, 7);
        let rotate_imm = inst.get_word(8, 11);
        let result = immediate.rotate_right(rotate_imm * 2);
        ctx.dis.push_word_end_arg(result, None);
        result
    } else {
        let rm = inst.get_byte(0, 3);
        ctx.dis.push_reg_end_arg(rm, None);
        arm.er(rm)
    };

    if !r {
        if field_mask.get_bit(0) && arm.cpsr().in_a_privileged_mode() {
            arm.switch_mode::<false>(
                ProcessorMode::from_bits_truncate(operand.get_bits(0, 4)),
                false,
            );
            arm.cpsr_mut().set_bits(5, 7, operand.get_bits(5, 7)); // 0:7
        }
        if field_mask.get_bit(1) && arm.cpsr().in_a_privileged_mode() {
            arm.cpsr_mut().set_bits(8, 15, operand.get_bits(8, 15)); // 8:15
        }
        if field_mask.get_bit(2) && arm.cpsr().in_a_privileged_mode() {
            arm.cpsr_mut().set_bits(16, 23, operand.get_bits(16, 23)); // 16:23
        }
        if field_mask.get_bit(3) {
            arm.cpsr_mut().set_bits(24, 31, operand.get_bits(24, 31)); // 24:31
        }
    } else if arm.cpsr().current_mode_has_spsr() {
        if field_mask.get_bit(0) {
            arm.get_spsr().set_bits(0, 7, operand.get_bits(0, 7)); // 0:7
        }
        if field_mask.get_bit(1) {
            arm.get_spsr().set_bits(8, 15, operand.get_bits(8, 15)); // 8:15
        }
        if field_mask.get_bit(2) {
            arm.get_spsr().set_bits(16, 23, operand.get_bits(16, 23)); // 16:23
        }
        if field_mask.get_bit(3) {
            arm.get_spsr().set_bits(24, 31, operand.get_bits(24, 31)); // 24:31
        }
    }

    1 // TODO: this is wrong
}
