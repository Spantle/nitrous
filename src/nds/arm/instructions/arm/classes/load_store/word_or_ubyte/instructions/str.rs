use crate::nds::arm::{
    instructions::arm::classes::load_store::word_or_ubyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// STR
#[inline(always)]
pub fn str(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.arm.write_word(
        ctx.bus,
        ctx.shared,
        address,
        ctx.arm.r()[ctx.inst.destination_register],
    );

    1
}
