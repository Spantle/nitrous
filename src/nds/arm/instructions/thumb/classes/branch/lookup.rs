use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    logger::LoggerTrait,
    Bits,
};

use super::instructions;

#[inline(always)]
pub fn lookup(
    arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let h = (inst_set >> 5) & 0b11;
    let offset_11 = ctx.inst.get_word(0, 10);
    match h {
        0b00 => {
            // Uncodonitional branch
            instructions::b_2(ctx)
        }
        0b01 => {
            if !arm_bool {
                // undefined instruction
                ctx.logger.log_warn("undefined blx variant 01");
                1
            } else {
                // BLX suffix
                ctx.dis.set_inst("BLX");
                let lr = ctx.arm.r()[14];
                let pc = (lr + (offset_11 << 1)) & 0xFFFFFFFC;
                ctx.dis.push_word_arg(pc);

                ctx.arm.set_r(14, (ctx.arm.r()[15] + 2) | 1);
                ctx.arm.set_r(15, pc);
                ctx.arm.cpsr_mut().set_thumb(false);
                1 // TODO: this is wrong
            }
        }
        0b10 => {
            // BL/BLX prefix
            ctx.dis.set_inst("BL(X)"); // ??????????????????
            let pc = (ctx.arm.r()[15] + 4) as i32;
            let lr = (pc + (offset_11.sign_extend(11) << 12)) as u32;
            ctx.dis.push_word_arg(lr);

            ctx.arm.set_r(14, lr);
            1 // TODO: this is wrong
        }
        0b11 => {
            // BL suffix
            ctx.dis.set_inst("BL");
            let lr = ctx.arm.r()[14];
            let pc = lr + (offset_11 << 1);
            ctx.dis.push_word_arg(pc);

            ctx.arm.set_r(14, (ctx.arm.r()[15] + 2) | 1);
            ctx.arm.set_r(15, pc);
            1 // TODO: this is wrong
        }
        _ => unreachable!(),
    }
}
