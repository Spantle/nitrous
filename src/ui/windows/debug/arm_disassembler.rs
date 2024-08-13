// TODO: this needs a good clean

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
    Arm,
    Thumb,
}

impl Display for DisassemblerInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisassemblerInstructionSet::Follow => write!(f, "Follow CPU"),
            DisassemblerInstructionSet::Arm => write!(f, "ARM"),
            DisassemblerInstructionSet::Thumb => write!(f, "THUMB"),
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
        egui::TopBottomPanel::top(id).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let selected_instruction_set = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_instruction_set,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_instruction_set,
                };
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
                            DisassemblerInstructionSet::Arm,
                            DisassemblerInstructionSet::Arm.to_string(),
                        );
                        ui.selectable_value(
                            selected_instruction_set,
                            DisassemblerInstructionSet::Thumb,
                            DisassemblerInstructionSet::Thumb.to_string(),
                        );
                    });

                let follow_pc = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_follow_pc,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_follow_pc,
                };
                ui.checkbox(follow_pc, "Follow PC");
            });

            ui.horizontal(|ui| {
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
                    let mut steps_remaining = step_amount.parse().unwrap_or(0);
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
                            steps_remaining -= 1;
                            if steps_remaining <= 0 {
                                break;
                            }
                        }
                    }
                }

                if ui
                    .button("Step over")
                    .on_hover_text("Will pause when the PC reaches the next instruction.")
                    .clicked()
                {
                    let pc = match ARM_BOOL {
                        ArmBool::ARM9 => self.emulator.arm9.r[15],
                        ArmBool::ARM7 => self.emulator.arm7.r[15],
                    };
                    let is_thumb = match ARM_BOOL {
                        ArmBool::ARM9 => self.emulator.arm9.cpsr.get_thumb(),
                        ArmBool::ARM7 => self.emulator.arm7.cpsr.get_thumb(),
                    };
                    let next_inst = if is_thumb { pc + 2 } else { pc + 4 };
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_step_until = Some(next_inst),
                        ArmBool::ARM7 => self.arm7_disassembler_step_until = Some(next_inst),
                    }

                    self.emulator.start();
                }

                if ui
                    .button("Step return")
                    .on_hover_text("Will pause when the PC reaches the LR. Use wisely!")
                    .clicked()
                {
                    let lr = match ARM_BOOL {
                        ArmBool::ARM9 => self.emulator.arm9.r[14],
                        ArmBool::ARM7 => self.emulator.arm7.r[14],
                    };
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_step_until = Some(lr),
                        ArmBool::ARM7 => self.arm7_disassembler_step_until = Some(lr),
                    }

                    self.emulator.start();
                }

                let is_running = self.emulator.is_running();
                let resume_text = if is_running { "Pause" } else { "Resume" };
                if ui.button(resume_text).clicked() {
                    if is_running {
                        self.emulator.pause();
                    } else {
                        self.emulator.start();
                    }
                }
            });

            ui.horizontal(|ui| {
                let jump_value = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_jump_value,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_jump_value,
                };
                ui.add(
                    egui::TextEdit::singleline(jump_value)
                        .hint_text("0xFFFFFFFF")
                        .desired_width(80.0)
                        .font(egui::TextStyle::Monospace),
                );

                if ui.button("Jump").clicked() {
                    let jump_value = Some(
                        u32::from_str_radix(jump_value.trim_start_matches("0x"), 16).unwrap_or(0),
                    );
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_jump_now = jump_value,
                        ArmBool::ARM7 => self.arm7_disassembler_jump_now = jump_value,
                    }
                }

                if ui.button("Jump to PC").clicked() {
                    let pc = match ARM_BOOL {
                        ArmBool::ARM9 => self.emulator.arm9.r[15],
                        ArmBool::ARM7 => self.emulator.arm7.r[15],
                    };
                    *jump_value = format!("{:08X}", pc);
                    match ARM_BOOL {
                        ArmBool::ARM9 => self.arm9_disassembler_jump_now = Some(pc),
                        ArmBool::ARM7 => self.arm7_disassembler_jump_now = Some(pc),
                    }
                }
                if ui.button("Add breakpoint").clicked() {
                    let jump_value =
                        u32::from_str_radix(jump_value.trim_start_matches("0x"), 16).unwrap_or(0);
                    let breakpoints = match ARM_BOOL {
                        ArmBool::ARM9 => &mut self.arm9_disassembler_breakpoints,
                        ArmBool::ARM7 => &mut self.arm7_disassembler_breakpoints,
                    };
                    breakpoints.push(jump_value);

                    let selected_breakpoint = match ARM_BOOL {
                        ArmBool::ARM9 => &mut self.arm9_disassembler_selected_breakpoint,
                        ArmBool::ARM7 => &mut self.arm7_disassembler_selected_breakpoint,
                    };
                    *selected_breakpoint = Some(breakpoints.len() - 1);
                }
            });

            ui.horizontal(|ui| {
                let breakpoints = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_breakpoints,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_breakpoints,
                };
                let selected_breakpoint = match ARM_BOOL {
                    ArmBool::ARM9 => &mut self.arm9_disassembler_selected_breakpoint,
                    ArmBool::ARM7 => &mut self.arm7_disassembler_selected_breakpoint,
                };
                let label = ui.label("Breakpoints");
                ui.add_visible_ui(true, |ui| {
                    ui.make_monospace();

                    let mut combobox = egui::ComboBox::from_id_source(label.id);
                    if let Some(selected_breakpoint) = selected_breakpoint {
                        combobox = combobox
                            .selected_text(format!("{:08X}", breakpoints[*selected_breakpoint]));
                    }
                    combobox.show_ui(ui, |ui| {
                        ui.make_monospace();

                        for (i, breakpoint) in breakpoints.iter().enumerate() {
                            ui.selectable_value(
                                selected_breakpoint,
                                Some(i),
                                format!("{:08X}", breakpoint),
                            );
                        }
                    });
                });

                if ui.button("Jump to BP").clicked() {
                    let selected_breakpoint = match ARM_BOOL {
                        ArmBool::ARM9 => &self.arm9_disassembler_selected_breakpoint,
                        ArmBool::ARM7 => &self.arm7_disassembler_selected_breakpoint,
                    };
                    if let Some(value) = selected_breakpoint {
                        let breakpoint = breakpoints[*value];
                        match ARM_BOOL {
                            ArmBool::ARM9 => self.arm9_disassembler_jump_now = Some(breakpoint),
                            ArmBool::ARM7 => self.arm7_disassembler_jump_now = Some(breakpoint),
                        }
                    }
                }

                if ui.button("Delete BP").clicked() {
                    let selected_breakpoint = match ARM_BOOL {
                        ArmBool::ARM9 => &mut self.arm9_disassembler_selected_breakpoint,
                        ArmBool::ARM7 => &mut self.arm7_disassembler_selected_breakpoint,
                    };
                    if let Some(value) = selected_breakpoint {
                        breakpoints.remove(*value);
                        if breakpoints.is_empty() {
                            *selected_breakpoint = None;
                        } else {
                            *selected_breakpoint = Some(*value - 1);
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
            DisassemblerInstructionSet::Arm => false,
            DisassemblerInstructionSet::Thumb => true,
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

        let follow_pc = match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler_follow_pc,
            ArmBool::ARM7 => self.arm7_disassembler_follow_pc,
        };
        if follow_pc && !(pc < arm_load_address || pc >= arm_load_address + arm_size as usize) {
            let pc_row = (pc - arm_load_address) / instruction_width;
            table_builder = table_builder.scroll_to_row(pc_row, Some(egui::Align::Center));
        };

        let jump_now = match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler_jump_now,
            ArmBool::ARM7 => self.arm7_disassembler_jump_now,
        };
        if let Some(jump_value) = jump_now {
            if jump_value >= (arm_load_address as u32)
                && jump_value < (arm_load_address as u32) + arm_size
            {
                let jump_row = (jump_value as usize - arm_load_address) / instruction_width;
                table_builder = table_builder.scroll_to_row(jump_row, Some(egui::Align::Center));
            }
        };

        match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler_jump_now = None,
            ArmBool::ARM7 => self.arm7_disassembler_jump_now = None,
        }
        let breakpoints = match ARM_BOOL {
            ArmBool::ARM9 => &mut self.arm9_disassembler_breakpoints,
            ArmBool::ARM7 => &mut self.arm7_disassembler_breakpoints,
        };

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
                        let is_breakpoint = breakpoints.contains(&(address as u32));
                        let text = format!("{:08X}", address);
                        let label = if is_breakpoint {
                            ui.colored_strong(egui::Color32::RED, text)
                        } else {
                            ui.label(text)
                        };
                        if label.clicked() {
                            ui.input(|r| {
                                if r.modifiers.ctrl {
                                    let selected_breakpoint = match ARM_BOOL {
                                        ArmBool::ARM9 => {
                                            &mut self.arm9_disassembler_selected_breakpoint
                                        }
                                        ArmBool::ARM7 => {
                                            &mut self.arm7_disassembler_selected_breakpoint
                                        }
                                    };

                                    if is_breakpoint {
                                        breakpoints.retain(|&x| x != address as u32);
                                        *selected_breakpoint = None;
                                    } else {
                                        breakpoints.push(address as u32);
                                        *selected_breakpoint = Some(breakpoints.len() - 1);
                                    }
                                };
                            });
                        }
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

                            let mut clickable = |ui: &mut egui::Ui, arg: &arm::models::Chunk| {
                                let label =
                                    ui.colored_label(match_color(&arg.kind), arg.value.to_string());
                                if arg.kind == arm::models::ChunkKind::Immediate && label.clicked()
                                {
                                    ui.input(|r| {
                                        if r.modifiers.ctrl {
                                            let jump_now = match ARM_BOOL {
                                                ArmBool::ARM9 => {
                                                    &mut self.arm9_disassembler_jump_now
                                                }
                                                ArmBool::ARM7 => {
                                                    &mut self.arm7_disassembler_jump_now
                                                }
                                            };
                                            *jump_now = Some(arg.raw);
                                        }
                                    });
                                }
                            };

                            if !disassembly.args.is_empty() {
                                ui.label(" ");

                                for arg in &disassembly.args {
                                    clickable(ui, arg);
                                }
                            }

                            if !disassembly.end_args.is_empty() {
                                if !disassembly.args.is_empty() {
                                    ui.label(", ");
                                } else {
                                    ui.label(" ");
                                }

                                for arg in &disassembly.end_args {
                                    clickable(ui, arg);
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
