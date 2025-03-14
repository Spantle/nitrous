use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait, ProcessorMode},
        ArmBool, ArmTrait,
    },
    logger::LoggerTrait,
};

// BKPT
#[inline(always)]
pub fn bkpt(arm_bool: bool, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("BKPT");

    // WARNING: the immediate value is ignored by the CPU, but is used by debuggers

    ctx.logger.log_warn("BKPT instruction executed");

    ctx.arm
        .set_mode_r(ProcessorMode::ABT, 1, ctx.arm.r()[15] + 4);
    ctx.arm.switch_mode::<false>(ProcessorMode::ABT, true);
    ctx.arm.cpsr_mut().set_thumb(false);
    ctx.arm.cpsr_mut().set_irq_interrupt(true);

    if arm_bool == ArmBool::ARM9 {
        ctx.arm.set_r(15, 0xFFFF000C);
    } else {
        ctx.arm.set_r(15, 0x0000000C);
    }

    1 // TODO: this is wrong
}
