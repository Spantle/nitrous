use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction},
};

// MSR
#[inline(always)]
pub fn msr(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MSR");

    let (inst, arm9) = (&mut ctx.inst, &mut ctx.arm9);

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
        arm9.er(rm)
    };

    if !r {
        if field_mask.get_bit(0) && arm9.cpsr().in_a_privileged_mode() {
            arm9.cpsr().set_bits(0, 7, operand.get_bits(0, 7)); // 0:7
        }
        if field_mask.get_bit(1) && arm9.cpsr().in_a_privileged_mode() {
            arm9.cpsr().set_bits(8, 15, operand.get_bits(8, 15)); // 8:15
        }
        if field_mask.get_bit(2) && arm9.cpsr().in_a_privileged_mode() {
            arm9.cpsr().set_bits(16, 23, operand.get_bits(16, 23)); // 16:23
        }
        if field_mask.get_bit(3) {
            arm9.cpsr().set_bits(24, 31, operand.get_bits(24, 31)); // 24:31
        }
    } else if arm9.cpsr().current_mode_has_spsr() {
        if field_mask.get_bit(0) {
            arm9.get_spsr().set_bits(0, 7, operand.get_bits(0, 7)); // 0:7
        }
        if field_mask.get_bit(1) {
            arm9.get_spsr().set_bits(8, 15, operand.get_bits(8, 15)); // 8:15
        }
        if field_mask.get_bit(2) {
            arm9.get_spsr().set_bits(16, 23, operand.get_bits(16, 23)); // 16:23
        }
        if field_mask.get_bit(3) {
            arm9.get_spsr().set_bits(24, 31, operand.get_bits(24, 31)); // 24:31
        }
    }

    1 // TODO: this is wrong
}
