use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// STRH
#[inline(always)]
pub fn strh(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    // if bit 0 of the address is 1, the data is UNPREDICTABLE

    let data = ctx.arm.er(ctx.inst.destination_register) as u16;
    ctx.bus.write_halfword(address, data);

    1
}