use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    logger::LoggerTrait,
    Bits,
};

// MRC
pub fn mrc(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("MRC");

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
    let value = match (crn, crm, opcode_2) {
        (1, 0, 0) => {
            // Control Register
            Some(arm.cp15().control_register.value())
        }
        (2, 0, 0) => {
            // PU Cachability Bits for Data/Unified Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Cachability Bits for Data/Unified Protection Region\"",
            );
            None
        }
        (2, 0, 1) => {
            // PU Cachability Bits for Instruction Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Cachability Bits for Instruction Protection Region\"",
            );
            None
        }
        (3, 0, 0) => {
            // PU Cache Write-Bufferability Bits for Data Protection Regions
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Cache Write-Bufferability Bits for Data Protection Regions\"",
            );
            None
        }
        (5, 0, 0) => {
            // PU Access Permission Data/Unified Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Access Permission Data/Unified Protection Region\"",
            );
            None
        }
        (5, 0, 1) => {
            // PU Access Permission Instruction Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Access Permission Instruction Protection Region\"",
            );
            None
        }
        (5, 0, 2) => {
            // PU Extended Access Permission Data/Unified Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Extended Access Permission Data/Unified Protection Region\"",
            );
            None
        }
        (5, 0, 3) => {
            // PU Extended Access Permission Instruction Protection Region
            ctx.logger.log_warn(
                "MRC: unimplemented \"PU Extended Access Permission Instruction Protection Region\"",
            );
            None
        }
        (6, 0..=7, 0) => {
            // PU Protection Unit Data/Unified Region
            ctx.logger
                .log_warn("MRC: unimplemented \"Protection Unit Data/Unified Region\"");
            None
        }
        (6, 0..=7, 1) => {
            // PU Protection Unit Instruction Region
            ctx.logger
                .log_warn("MRC: unimplemented \"Protection Unit Instruction Region\"");
            None
        }
        (7, _, _) => {
            // Cache Commands and Halt Function
            ctx.logger
                .log_warn("MRC: unimplemented \"Cache Commands and Halt Function\"");
            None
        }
        (9, 0, 0) => {
            // Cache Data Lockdown
            ctx.logger
                .log_warn("MRC: unimplemented \"Cache Data Lockdown\"");
            None
        }
        (9, 0, 1) => {
            // Cache Instruction Lockdown
            ctx.logger
                .log_warn("MRC: unimplemented \"Cache Instruction Lockdown\"");
            None
        }
        (9, 1, 0) => {
            // TCM Data TCM Base and Virtual Size
            Some(arm.cp15().data_tcm_reg)
        }
        (9, 1, 1) => {
            // Instruction TCM Size/Base
            Some(arm.cp15().inst_tcm_reg)
        }
        _ => {
            ctx.logger.log_warn(format!(
                "MRC: unhandled instruction: CP{},{},{},CR{},CR{},{}",
                cp_num, opcode_1, rd, crn, crm, opcode_2
            ));
            None
        }
    };

    if let Some(value) = value {
        if rd == 15 {
            let cpsr = arm.cpsr_mut();
            cpsr.set_negative(value.get_bit(31));
            cpsr.set_zero(value.get_bit(30));
            cpsr.set_carry(value.get_bit(29));
            cpsr.set_overflow(value.get_bit(28));
        } else {
            arm.set_r(rd, value);
        }
    }

    1 // TODO: this is wrong
}
