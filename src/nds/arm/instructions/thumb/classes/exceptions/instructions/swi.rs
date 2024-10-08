use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait, ProcessorMode},
    ArmBool, ArmTrait,
};

// SWI
pub fn swi(arm_bool: bool, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SWI");

    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_word_arg(immed_8);

    ctx.arm
        .set_mode_r(ProcessorMode::SVC, 1, ctx.arm.r()[15] + 2);
    ctx.arm.switch_mode::<false>(ProcessorMode::SVC, true);
    ctx.arm.cpsr_mut().set_thumb(false);
    ctx.arm.cpsr_mut().set_irq_interrupt(true);

    if arm_bool == ArmBool::ARM9 {
        ctx.arm.set_r(15, 0xFFFF0008);
    } else {
        ctx.arm.set_r(15, 0x00000008);
    }

    1 // TODO: this is wrong
}
