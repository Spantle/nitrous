use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait, ProcessorMode},
};

// SWI
pub fn swi(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SWI");

    let immed_24 = ctx.inst.get_word(0, 23);
    ctx.dis.push_word_arg(immed_24);

    ctx.arm
        .set_mode_r(ProcessorMode::SVC, 1, ctx.arm.r()[15] + 4);
    ctx.arm.switch_mode::<false>(ProcessorMode::SVC, true);
    ctx.arm.cpsr_mut().set_thumb(false);
    ctx.arm.cpsr_mut().set_irq_interrupt(true);
    ctx.arm.set_r(15, 0xFFFF0008);

    1 // TODO: this is wrong
}
