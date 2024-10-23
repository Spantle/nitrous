use crate::nds::arm::{
    instructions::arm::classes::load_store::halfword_or_ibyte::LoadStoreInstruction,
    models::{Context, ContextTrait},
    ArmTrait,
};

// LDRSB
#[inline(always)]
pub fn ldrsb(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    let data = ctx.arm.read_byte(ctx.bus, ctx.shared, address) as i8 as i32 as u32;
    ctx.arm.set_r(ctx.inst.destination_register, data);

    1
}
