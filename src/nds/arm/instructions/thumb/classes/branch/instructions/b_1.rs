use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::{conditions::calculate_cond, thumb::Instruction},
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// B (1)
pub fn b_1(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("B");

    let pc = (ctx.arm.r()[15] + 4) as i32;
    let signed_immed_8 = ctx.inst.get_word(0, 7);
    let signed_immed_8 = (pc + (sign_extend_8_to_32(signed_immed_8) << 1)) as u32;
    ctx.dis.push_word_arg(signed_immed_8);

    let cond = ((inst_set >> 2) & 0b1111) as u8;
    let cond_result = calculate_cond(cond, ctx);
    if !cond_result {
        return 1;
    }

    ctx.arm.set_r(15, signed_immed_8);

    1 // TODO: this is wrong
}

fn sign_extend_8_to_32(value: u32) -> i32 {
    let sign_bit = value.get_bit(7);

    let extended_value = if sign_bit {
        (value | 0xFFFFFF80) as i32
    } else {
        value as i32
    };

    extended_value
}
