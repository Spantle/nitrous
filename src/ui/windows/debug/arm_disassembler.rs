use std::fmt::Display;

use crate::{
    nds::{
        arm::{self, bus, instructions, models::Disassembly, ArmBool},
        logger, shared, CycleState,
    },
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

#[derive(PartialEq)]
pub enum DisassemblerInstructionSet {
    Follow,
    ARM,
    THUMB,
}

impl Display for DisassemblerInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisassemblerInstructionSet::Follow => write!(f, "Follow CPU"),
            DisassemblerInstructionSet::ARM => write!(f, "ARM"),
            DisassemblerInstructionSet::THUMB => write!(f, "THUMB"),
        }
    }
}

impl NitrousGUI {
    pub fn show_arm_disassembler<const ARM_BOOL: bool>(&mut self, ctx: &egui::Context) {
        let mut arm_disassembler = match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler,
            ArmBool::ARM7 => self.arm7_disassembler,
        };
        let title = match ARM_BOOL {
            ArmBool::ARM9 => "ARM9 Disassembler",
            ArmBool::ARM7 => "ARM7 Disassembler",
        };

        egui::Window::new_nitrous(title, ctx)
            .open(&mut arm_disassembler)
            .show(ctx, |ui| {
                self.render_navbar::<ARM_BOOL>(ui);
                self.render_instructions::<ARM_BOOL>(ui);
            });

        match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler = arm_disassembler,
            ArmBool::ARM7 => self.arm7_disassembler = arm_disassembler,
        };
    }

    fn render_navbar<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        let id = match ARM_BOOL {
            ArmBool::ARM9 => "arm9_disassembler_navbar",
            ArmBool::ARM7 => "arm7_disassembler_navbar",
        };
        let selected_instruction_set = match ARM_BOOL {
            ArmBool::ARM9 => &mut self.arm9_disassembler_instruction_set,
            ArmBool::ARM7 => &mut self.arm7_disassembler_instruction_set,
        };
        let follow_pc = match ARM_BOOL {
            ArmBool::ARM9 => &mut self.arm9_disassembler_follow_pc,
            ArmBool::ARM7 => &mut self.arm7_disassembler_follow_pc,
        };
        let jump_value = match ARM_BOOL {
            ArmBool::ARM9 => &mut self.arm9_disassembler_jump_value,
            ArmBool::ARM7 => &mut self.arm7_disassembler_jump_value,
        };

        egui::TopBottomPanel::top(id).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Instruction Set");
                egui::ComboBox::from_id_source(label.id)
                    .selected_text(selected_instruction_set.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            selected_instruction_set,
                            DisassemblerInstructionSet::Follow,
                            DisassemblerInstructionSet::Follow.to_string(),
                        );
                        ui.selectable_value(
                            selected_instruction_set,
                            DisassemblerInstructionSet::ARM,
                            DisassemblerInstructionSet::ARM.to_string(),
                        );
                        ui.selectable_value(
                            selected_instruction_set,
                            DisassemblerInstructionSet::THUMB,
                            DisassemblerInstructionSet::THUMB.to_string(),
                        );
                    });

                ui.checkbox(follow_pc, "Follow PC");
            });

            ui.horizontal(|ui| {
                ui.label("Jump to");
                ui.add(
                    egui::TextEdit::singleline(jump_value)
                        .hint_text("FFFFFFFF")
                        .desired_width(60.0)
                        .font(egui::TextStyle::Monospace),
                );
                if ui.button("Jump").clicked() {
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_jump_now = true,
                        ArmBool::ARM7 => self.arm7_disassembler_jump_now = true,
                    }
                }
                if ui.button("Jump to PC").clicked() {
                    let pc = match ARM_BOOL {
                        ArmBool::ARM9 => self.emulator.arm9.r[15],
                        ArmBool::ARM7 => self.emulator.arm7.r[15],
                    };
                    *jump_value = format!("{:08X}", pc);
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_jump_now = true,
                        ArmBool::ARM7 => self.arm7_disassembler_jump_now = true,
                    }
                }

                let step_amount = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_step_amount,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_step_amount,
                };
                ui.add(
                    egui::TextEdit::singleline(step_amount)
                        .hint_text("1")
                        .desired_width(40.0),
                );

                if ui.button("Step").clicked() {
                    match ARM_BOOL {
                        ArmBool::ARM9 => {
                            self.arm9_disassembler_steps_remaining =
                                step_amount.parse().unwrap_or(0);
                        }
                        ArmBool::ARM7 => {
                            self.arm7_disassembler_steps_remaining =
                                step_amount.parse().unwrap_or(0);
                        }
                    }

                    loop {
                        self.emulator.clock();

                        let mut ran = false;
                        match ARM_BOOL {
                            ArmBool::ARM9 => {
                                if self.emulator.cycle_state == CycleState::Arm9_2
                                    || self.emulator.cycle_state == CycleState::Arm7
                                {
                                    ran = true;
                                }
                            }
                            ArmBool::ARM7 => {
                                if self.emulator.cycle_state == CycleState::Arm9_1 {
                                    ran = true;
                                }
                            }
                        }

                        if ran {
                            let steps_remaining = match ARM_BOOL {
                                ArmBool::ARM9 => self.arm9_disassembler_steps_remaining,
                                ArmBool::ARM7 => self.arm7_disassembler_steps_remaining,
                            };

                            if steps_remaining > 0 {
                                match ARM_BOOL {
                                    ArmBool::ARM9 => self.arm9_disassembler_steps_remaining -= 1,
                                    ArmBool::ARM7 => self.arm7_disassembler_steps_remaining -= 1,
                                };
                            } else {
                                break;
                            }
                        }
                    }
                }
            });
        });
    }

    fn render_instructions<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        let mut fake_bus = bus::FakeBus;
        let mut fake_shared = shared::Shared::new_fake();

        ui.make_monospace();

        let (arm_load_address, arm_size) = match ARM_BOOL {
            ArmBool::ARM9 => (
                self.emulator.shared.cart.arm9_load_address,
                self.emulator.shared.cart.arm9_size,
            ),
            ArmBool::ARM7 => (
                self.emulator.shared.cart.arm7_load_address,
                self.emulator.shared.cart.arm7_size,
            ),
        };
        let mem = if arm_size == 0 {
            vec![]
        } else {
            match ARM_BOOL {
                ArmBool::ARM9 => self.emulator.arm9.read_bulk(
                    &mut self.emulator.bus9,
                    &mut self.emulator.shared,
                    arm_load_address,
                    arm_size,
                ),
                ArmBool::ARM7 => self.emulator.arm7.read_bulk(
                    &mut self.emulator.bus7,
                    &mut self.emulator.shared,
                    arm_load_address,
                    arm_size,
                ),
            }
        };

        let instruction_set = match ARM_BOOL {
            ArmBool::ARM9 => &self.arm9_disassembler_instruction_set,
            ArmBool::ARM7 => &self.arm7_disassembler_instruction_set,
        };
        let is_thumb = match instruction_set {
            DisassemblerInstructionSet::Follow => match ARM_BOOL {
                ArmBool::ARM9 => self.emulator.arm9.cpsr.get_thumb(),
                ArmBool::ARM7 => self.emulator.arm7.cpsr.get_thumb(),
            },
            DisassemblerInstructionSet::ARM => false,
            DisassemblerInstructionSet::THUMB => true,
        };
        let instruction_width = if is_thumb { 2 } else { 4 };

        let pc = match ARM_BOOL {
            ArmBool::ARM9 => self.emulator.arm9.r[15],
            ArmBool::ARM7 => self.emulator.arm7.r[15],
        } as usize;
        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        let total_rows = mem.len() / instruction_width;
        let arm_load_address = arm_load_address as usize;
        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::remainder());

        if self.emulator.is_running() {
            if pc < arm_load_address || pc >= arm_load_address + arm_size as usize {
                return;
            }
            let pc_row = (pc - arm_load_address) / instruction_width;

            let follow_pc = match ARM_BOOL {
                ArmBool::ARM9 => self.arm9_disassembler_follow_pc,
                ArmBool::ARM7 => self.arm7_disassembler_follow_pc,
            };
            if follow_pc {
                table_builder = table_builder.scroll_to_row(pc_row, Some(egui::Align::Center));
            };
        }

        let jump_now = match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler_jump_now,
            ArmBool::ARM7 => self.arm7_disassembler_jump_now,
        };
        if jump_now {
            let jump_value = match ARM_BOOL {
                ArmBool::ARM9 => &self.arm9_disassembler_jump_value,
                ArmBool::ARM7 => &self.arm7_disassembler_jump_value,
            };
            let jump_value = u32::from_str_radix(jump_value, 16).unwrap_or(0);
            let jump_row = (jump_value as usize - arm_load_address) / instruction_width;
            table_builder = table_builder.scroll_to_row(jump_row, Some(egui::Align::Center));
        };

        match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler_jump_now = false,
            ArmBool::ARM7 => self.arm7_disassembler_jump_now = false,
        }

        table_builder
            .header(height, |mut header| {
                header.col(|c| {
                    c.strong("Address");
                });
                header.col(|c| {
                    c.strong("Instruction");
                });
                header.col(|c| {
                    c.strong("Disassembly");
                });
            })
            .body(|body| {
                body.rows(height, total_rows, |mut row| {
                    let start = row.index() * instruction_width;
                    let address = arm_load_address + start;
                    let inst = if is_thumb {
                        u16::from_le_bytes([mem[start], mem[start + 1]]) as u32
                    } else {
                        u32::from_le_bytes([
                            mem[start],
                            mem[start + 1],
                            mem[start + 2],
                            mem[start + 3],
                        ])
                    };

                    row.set_selected(address == pc);

                    let mut disassembly = Disassembly::default();
                    if is_thumb {
                        instructions::thumb::lookup_instruction::<{ ARM_BOOL }>(
                            &mut arm::models::Context::new(
                                (inst as u16).into(),
                                &mut arm::FakeArm::new(address as u32),
                                &mut fake_bus,
                                &mut fake_shared,
                                &mut disassembly,
                                &mut logger::FakeLogger,
                            ),
                        )
                    } else {
                        instructions::arm::lookup_instruction::<{ ARM_BOOL }>(
                            &mut arm::models::Context::new(
                                inst.into(),
                                &mut arm::FakeArm::new(address as u32),
                                &mut fake_bus,
                                &mut fake_shared,
                                &mut disassembly,
                                &mut logger::FakeLogger,
                            ),
                        )
                    };
                    row.col(|ui| {
                        ui.label(format!("{:08X}", address));
                    });
                    row.col(|ui| {
                        if is_thumb {
                            ui.label(format!("{:04X}", inst as u16));
                        } else {
                            ui.label(format!("{:08X}", inst));
                        }
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            ui.colored_label(
                                egui::Color32::from_rgb(250, 40, 170),
                                disassembly.inst,
                            );
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 215, 0),
                                disassembly
                                    .cond
                                    .unwrap_or_default()
                                    .iter()
                                    .collect::<String>(),
                            );
                            ui.colored_label(
                                egui::Color32::from_rgb(30, 160, 255),
                                disassembly.inst_suffix,
                            );

                            if !disassembly.args.is_empty() {
                                ui.label(" ");

                                for arg in &disassembly.args {
                                    ui.colored_label(match_color(&arg.kind), arg.value.to_string());
                                }
                            }

                            if !disassembly.end_args.is_empty() {
                                ui.label(", ");

                                for arg in &disassembly.end_args {
                                    ui.colored_label(match_color(&arg.kind), arg.value.to_string());
                                }
                            }
                        });
                    });
                })
            })
    }
}

fn match_color(kind: &arm::models::ChunkKind) -> egui::Color32 {
    match kind {
        arm::models::ChunkKind::Register => egui::Color32::from_rgb(190, 240, 250),
        arm::models::ChunkKind::Immediate => egui::Color32::from_rgb(250, 90, 70),
        arm::models::ChunkKind::Modifier => egui::Color32::from_rgb(210, 110, 210),
        arm::models::ChunkKind::Punctuation => egui::Color32::from_rgb(140, 140, 140),
    }
}
