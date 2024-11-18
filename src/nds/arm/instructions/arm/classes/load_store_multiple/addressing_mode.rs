use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    IfElse,
};

pub fn parse(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> (u32, u32, u32) {
    let arm = &mut ctx.arm;

    let is_excluded = inst_set >> 4 & 1 == 1; // P
    let is_upwards = inst_set >> 3 & 1 == 1; // U
    let is_writeback = inst_set >> 1 & 1 == 1; // W

    let destination = ctx.inst.get_byte(16, 19); // Rn
    if is_writeback {
        ctx.dis.push_reg_arg(destination, Some("!"));
    } else {
        ctx.dis.push_reg_arg(destination, None);
    }

    let register_list = ctx.inst.get_halfword(0, 15);
    let number_of_set_bits = register_list.count_ones();
    let rn = arm.eru(destination);
    match (is_excluded, is_upwards) {
        (false, true) => {
            // increment after
            ctx.dis.set_inst_suffix("IA");
            let start_address = rn;
            let end_address = rn.wrapping_add(number_of_set_bits * 4).wrapping_sub(4);
            let writeback = is_writeback.if_else(rn.wrapping_add(number_of_set_bits * 4), 0);
            (start_address, end_address, writeback)
        }
        (true, true) => {
            // increment before
            ctx.dis.set_inst_suffix("IB");
            let start_address = rn.wrapping_add(4);
            let end_address = rn.wrapping_add(number_of_set_bits * 4);
            let writeback = is_writeback.if_else(rn.wrapping_add(number_of_set_bits * 4), 0);
            (start_address, end_address, writeback)
        }
        (false, false) => {
            // decrement after
            ctx.dis.set_inst_suffix("DA");
            let start_address = rn.wrapping_sub(number_of_set_bits * 4).wrapping_add(4);
            let end_address = rn;
            let writeback = is_writeback.if_else(rn.wrapping_sub(number_of_set_bits * 4), 0);
            (start_address, end_address, writeback)
        }
        (true, false) => {
            // decrement before
            ctx.dis.set_inst_suffix("DB");
            let start_address = rn.wrapping_sub(number_of_set_bits * 4);
            let end_address = rn.wrapping_sub(4);
            let writeback = is_writeback.if_else(rn.wrapping_sub(number_of_set_bits * 4), 0);
            (start_address, end_address, writeback)
        }
    }
}
