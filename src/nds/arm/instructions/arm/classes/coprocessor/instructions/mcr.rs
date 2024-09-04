use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    logger::LoggerTrait,
    Bits,
};

// MCR
pub fn mcr(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MCR");

    let (arm, inst) = (&mut ctx.arm, &ctx.inst);

    // arm
    let cp_num = inst.get_byte(8, 11);
    let rd = inst.get_byte(12, 15);

    // cp15
    // assuming that it's all CP15
    // call that "performance" :clueless:
    let opcode_1 = inst.get_byte(21, 23); // cpopc
    let crn = inst.get_byte(16, 19); // cn
    let crm = inst.get_byte(0, 3); // cm
    let opcode_2 = inst.get_byte(5, 7); // cp

    ctx.dis.push_str_arg(&format!("CP{}", cp_num));
    ctx.dis.push_str_end_arg(&opcode_1.to_string(), None);
    ctx.dis.push_reg_end_arg(rd, Some(", "));
    ctx.dis.push_str_end_arg(&format!("CR{}", crn), Some(", "));
    ctx.dis.push_str_end_arg(&format!("CR{}", crm), Some(", "));
    ctx.dis.push_str_end_arg(&opcode_2.to_string(), Some(", "));

    // opcode_1 always appears to be 0
    match (crn, crm, opcode_2) {
        (1, 0, 0) => {
            // Control Register
            arm.cp15_mut().control_register = arm.er(rd).into();

            ctx.logger.log_debug(format!(
                "CP15 Control Register updated: 0x{:08X}",
                arm.cp15_mut().control_register.value(),
            ));
        }
        (2, 0, 0) => {
            // PU Cachability Bits for Data/Unified Protection Region
            // ctx.logger.log_warn(
            //     "MCR: unimplemented \"PU Cachability Bits for Data/Unified Protection Region\"",
            // );
        }
        (2, 0, 1) => {
            // PU Cachability Bits for Instruction Protection Region
            // ctx.logger.log_warn(
            //     "MCR: unimplemented \"PU Cachability Bits for Instruction Protection Region\"",
            // );
        }
        (5, 0, 2) => {
            // PU Extended Access Permission Data/Unified Protection Region
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"PU Extended Access Permission Data/Unified Protection Region\"");
        }
        (5, 0, 3) => {
            // PU Extended Access Permission Instruction Protection Region
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"PU Extended Access Permission Instruction Protection Region\"");
        }
        (3, 0, 0) => {
            // PU Cache Write-Bufferability Bits for Data Protection Regions
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"PU Cache Write-Bufferability Bits for Data Protection Regions\"");
        }
        (6, 0..=7, 0) => {
            // Protection Unit Data/Unified Region
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"Protection Unit Data/Unified Region\"");
        }
        (6, 0..=7, 1) => {
            // Protection Unit Instruction Region
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"Protection Unit Instruction Region\"");
        }
        (7, 0, 4) => {
            // Wait For Interrupt (Halt)
            arm.halt();
        }
        (7, 5, 0) => {
            // Invalidate Entire Instruction Cache
            arm.cp15_mut().inst_tcm = vec![0; arm.cp15().inst_tcm_size as usize];
        }
        (7, 6, 0) => {
            // Invalidate Entire Data Cache
            arm.cp15_mut().data_tcm = vec![0; arm.cp15().data_tcm_size as usize];
        }
        (7, 10, 4) => {
            // ctx.logger
            //     .log_warn("MCR: unimplemented \"Drain Write Buffer\"");
        }
        (9, 1, 0) => {
            // Data TCM Base and Virtual Size
            let rd = arm.er(rd);
            let virtual_size = 512 << rd.get_bits(1, 5);
            let region_base = rd.get_bits(12, 31) << 12;
            arm.cp15_mut().data_tcm_reg = rd;
            arm.cp15_mut().data_tcm_base = region_base;
            arm.cp15_mut().data_tcm_size = virtual_size;

            ctx.logger.log_debug(format!(
                "Data TCM moved: 0x{:08X}-0x{:08X}, size: 0x{:08X}",
                region_base,
                region_base + virtual_size,
                virtual_size
            ));
        }
        (9, 1, 1) => {
            // Instruction TCM Base and Virtual Size
            // Instruction TCM Base is FIXED but w/e
            let rd = arm.er(rd);
            let virtual_size = 512 << rd.get_bits(1, 5);
            let region_base = rd.get_bits(12, 31) << 12;
            arm.cp15_mut().inst_tcm_reg = rd;
            arm.cp15_mut().inst_tcm_base = region_base;
            arm.cp15_mut().inst_tcm_size = virtual_size;

            ctx.logger.log_debug(format!(
                "Instruction TCM moved: 0x{:08X}-0x{:08X}, size: 0x{:08X}",
                region_base,
                region_base + virtual_size,
                virtual_size
            ));
        }
        _ => {
            ctx.logger.log_warn(format!(
                "MCR: unhandled instruction: CP{},{},{},CR{},CR{},{}",
                cp_num, opcode_1, rd, crn, crm, opcode_2
            ));
        }
    }

    1 // TODO: this is wrong
}
