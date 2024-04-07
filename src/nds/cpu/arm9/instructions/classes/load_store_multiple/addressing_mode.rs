use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

pub fn parse(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> (u32, u32) {
    let arm9 = &mut ctx.arm9;

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
    let rn = arm9.eru(destination);
    let (start_address, end_address) = match (is_excluded, is_upwards) {
        (false, true) => {
            // increment after
            ctx.dis.set_inst_suffix("IA");
            let start_address = rn;
            let end_address = rn.wrapping_add((number_of_set_bits * 4) - 4);
            if is_writeback {
                arm9.r()[destination] = rn.wrapping_add(number_of_set_bits * 4);
            }
            (start_address, end_address)
        }
        (true, true) => {
            // increment before
            ctx.dis.set_inst_suffix("IB");
            let start_address = rn.wrapping_add(4);
            let end_address = rn.wrapping_add(number_of_set_bits * 4);
            if is_writeback {
                arm9.r()[destination] = rn.wrapping_add(number_of_set_bits * 4);
            }
            (start_address, end_address)
        }
        (false, false) => {
            // decrement after
            ctx.dis.set_inst_suffix("DA");
            let start_address = rn.wrapping_sub((number_of_set_bits * 4) + 4);
            let end_address = rn;
            if is_writeback {
                arm9.r()[destination] = rn.wrapping_sub(number_of_set_bits * 4);
            }
            (start_address, end_address)
        }
        (true, false) => {
            // decrement before
            ctx.dis.set_inst_suffix("DB");
            let start_address = rn.wrapping_sub(number_of_set_bits * 4);
            let end_address = rn.wrapping_sub(4);
            if is_writeback {
                arm9.r()[destination] = rn.wrapping_sub(number_of_set_bits * 4);
            }
            (start_address, end_address)
        }
    };

    (start_address, end_address)
}
