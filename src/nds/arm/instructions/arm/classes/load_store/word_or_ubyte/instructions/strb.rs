use crate::nds::arm::{
    instructions::arm::classes::load_store::word_or_ubyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// STRB
#[inline(always)]
pub fn strb(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.arm.write_byte(
        ctx.bus,
        ctx.shared,
        address,
        ctx.arm.eru(ctx.inst.destination_register) as u8,
    );

    1
}
