// TODO: this needs a good clean

use crate::{
    nds::{
        arm::{self, instructions, models::Disassembly, ArmBool, ArmInternalRW, ArmTrait},
        bus, dma, logger, shared, CycleState, Emulator,
    },
    ui::{NitrousUI, NitrousWindow},
};

use super::{
    models::{match_color, DisassemblerInstructionSet},
    ArmDisassemblerWindow,
};

impl ArmDisassemblerWindow {
    pub fn check_breakpoints<const ARM_BOOL: bool>(&mut self, emulator: &mut Emulator) -> bool {
        let pc = match ARM_BOOL {
            ArmBool::ARM9 => emulator.arm9.r[15],
            ArmBool::ARM7 => emulator.arm7.r[15],
        };

        if self.open {
            if self.step_until == Some(pc) {
                emulator.pause();
                self.step_until = None;

                return true;
            }

            if self.breakpoints.contains(&pc) {
                emulator.pause();
                self.selected_breakpoint =
                    Some(self.breakpoints.iter().position(|&x| x == pc).unwrap());

                return true;
            }
        }

        false
    }

    pub fn show<const ARM_BOOL: bool>(&mut self, emulator: &mut Emulator, ctx: &egui::Context) {
        let mut open = self.open;
        let title = match ARM_BOOL {
            ArmBool::ARM9 => "ARM9 Disassembler",
            ArmBool::ARM7 => "ARM7 Disassembler",
        };

        egui::Window::new_nitrous(title, ctx)
            .open(&mut open)
            .show(ctx, |ui| {
                self.render_navbar::<ARM_BOOL>(emulator, ui);
                self.render_instructions::<ARM_BOOL>(emulator, ui);
            });

        self.open = open;
    }

    fn render_navbar<const ARM_BOOL: bool>(&mut self, emulator: &mut Emulator, ui: &mut egui::Ui) {
        let id = match ARM_BOOL {
            ArmBool::ARM9 => "arm9_disassembler_navbar",
            ArmBool::ARM7 => "arm7_disassembler_navbar",
        };

        egui::TopBottomPanel::top(id).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Instruction Set");
                egui::ComboBox::from_id_source(label.id)
                    .selected_text(self.instruction_set.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.instruction_set,
                            DisassemblerInstructionSet::Follow,
                            DisassemblerInstructionSet::Follow.to_string(),
                        );
                        ui.selectable_value(
                            &mut self.instruction_set,
                            DisassemblerInstructionSet::Arm,
                            DisassemblerInstructionSet::Arm.to_string(),
                        );
                        ui.selectable_value(
                            &mut self.instruction_set,
                            DisassemblerInstructionSet::Thumb,
                            DisassemblerInstructionSet::Thumb.to_string(),
                        );
                    });

                ui.checkbox(&mut self.follow_pc, "Follow PC");

                if ui.button("Generate Stacktrace").clicked() {
                    let (stacktrace, r, log_source) = match ARM_BOOL {
                        ArmBool::ARM9 => (
                            &emulator.arm9.stacktrace,
                            &emulator.arm9.r(),
                            logger::LogSource::Arm9(0),
                        ),
                        ArmBool::ARM7 => (
                            &emulator.arm7.stacktrace,
                            &emulator.arm7.r(),
                            logger::LogSource::Arm7(0),
                        ),
                    };

                    let stacktrace = stacktrace.generate(r, "Requested by user".to_string());
                    logger::debug_release(log_source, stacktrace);
                }
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.step_amount)
                        .hint_text("1")
                        .desired_width(40.0),
                );

                if ui.button("Step").clicked() {
                    let mut steps_remaining = self.step_amount.parse().unwrap_or(0);
                    loop {
                        emulator.step();

                        let mut ran = false;
                        match ARM_BOOL {
                            ArmBool::ARM9 => {
                                if emulator.cycle_state == CycleState::Arm9_2
                                    || emulator.cycle_state == CycleState::Arm7
                                {
                                    ran = true;
                                }
                            }
                            ArmBool::ARM7 => {
                                if emulator.cycle_state == CycleState::Arm9_1 {
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
                        ArmBool::ARM9 => emulator.arm9.r[15],
                        ArmBool::ARM7 => emulator.arm7.r[15],
                    };
                    let is_thumb = match ARM_BOOL {
                        ArmBool::ARM9 => emulator.arm9.cpsr.get_thumb(),
                        ArmBool::ARM7 => emulator.arm7.cpsr.get_thumb(),
                    };
                    let next_inst = if is_thumb { pc + 2 } else { pc + 4 };
                    self.step_until = Some(next_inst);

                    emulator.start();
                }

                if ui
                    .button("Step return")
                    .on_hover_text("Will pause when the PC reaches the LR. Use wisely!")
                    .clicked()
                {
                    let is_thumb = match ARM_BOOL {
                        ArmBool::ARM9 => emulator.arm9.cpsr.get_thumb(),
                        ArmBool::ARM7 => emulator.arm7.cpsr.get_thumb(),
                    };
                    let lr = match ARM_BOOL {
                        ArmBool::ARM9 => emulator.arm9.r[14],
                        ArmBool::ARM7 => emulator.arm7.r[14],
                    };
                    let lr = if is_thumb { lr & 0xFFFFFFFE } else { lr };
                    self.step_until = Some(lr);

                    emulator.start();
                }

                let is_running = emulator.is_running();
                let resume_text = if is_running { "Pause" } else { "Resume" };
                if ui.button(resume_text).clicked() {
                    if is_running {
                        emulator.pause();
                    } else {
                        emulator.start();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.jump_value)
                        .hint_text("0xFFFFFFFF")
                        .desired_width(80.0)
                        .font(egui::TextStyle::Monospace),
                );

                if ui.button("Jump").clicked() {
                    let jump_value = Some(
                        u32::from_str_radix(self.jump_value.trim_start_matches("0x"), 16)
                            .unwrap_or(0),
                    );
                    self.jump_now = jump_value;
                }

                if ui.button("Jump to PC").clicked() {
                    let pc = match ARM_BOOL {
                        ArmBool::ARM9 => emulator.arm9.r[15],
                        ArmBool::ARM7 => emulator.arm7.r[15],
                    };
                    self.jump_value = format!("{:08X}", pc);
                    self.jump_now = Some(pc);
                }
                if ui.button("Add breakpoint").clicked() {
                    let jump_value =
                        u32::from_str_radix(self.jump_value.trim_start_matches("0x"), 16)
                            .unwrap_or(0);
                    self.breakpoints.push(jump_value);

                    self.selected_breakpoint = Some(self.breakpoints.len() - 1);
                }
            });

            ui.horizontal(|ui| {
                let label = ui.label("Breakpoints");
                ui.add_visible_ui(true, |ui| {
                    ui.make_monospace();

                    let mut combobox = egui::ComboBox::from_id_source(label.id);
                    if let Some(selected_breakpoint) = self.selected_breakpoint {
                        combobox = combobox.selected_text(format!(
                            "{:08X}",
                            self.breakpoints[selected_breakpoint]
                        ));
                    }
                    combobox.show_ui(ui, |ui| {
                        ui.make_monospace();

                        for (i, breakpoint) in self.breakpoints.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_breakpoint,
                                Some(i),
                                format!("{:08X}", breakpoint),
                            );
                        }
                    });
                });

                if ui.button("Jump to BP").clicked() {
                    if let Some(value) = self.selected_breakpoint {
                        let breakpoint = self.breakpoints[value];
                        self.jump_now = Some(breakpoint);
                    }
                }

                if ui.button("Delete BP").clicked() {
                    if let Some(value) = self.selected_breakpoint {
                        self.breakpoints.remove(value);
                        if self.breakpoints.is_empty() {
                            self.selected_breakpoint = None;
                        } else {
                            self.selected_breakpoint = Some(value.max(1) - 1);
                        }
                    }
                }
            });
        });
    }

    fn render_instructions<const ARM_BOOL: bool>(
        &mut self,
        emulator: &mut Emulator,
        ui: &mut egui::Ui,
    ) {
        let mut fake_bus = bus::FakeBus::default();
        let mut fake_shared = shared::Shared::new_fake();
        let mut fake_dma = dma::Dma::default(); // TODO: probably a good idea to make a fake one in the future with no-op functions

        ui.make_monospace();

        let (pc, bios_size, arm_load_address, arm_size) = match ARM_BOOL {
            ArmBool::ARM9 => (
                emulator.arm9.r[15] as usize,
                emulator.bus9.bios.len(),
                emulator.shared.cart.metadata.arm9_load_address as usize,
                emulator.shared.cart.metadata.arm9_size as usize,
            ),
            ArmBool::ARM7 => (
                emulator.arm7.r[15] as usize,
                0,
                emulator.shared.cart.metadata.arm7_load_address as usize,
                emulator.shared.cart.metadata.arm7_size as usize,
            ),
        };

        let (mem_start, mem_size) = if pc >= arm_load_address && pc < arm_load_address + arm_size {
            (arm_load_address, arm_size)
        } else if pc >= 0xFFFF0000 && pc < 0xFFFF0000 + bios_size {
            (0xFFFF0000, bios_size)
        } else if !ARM_BOOL {
            if (0x03000000..=0x037FFFFF).contains(&pc) {
                (0x03000000, 0x800000)
            } else if (0x03800000..=0x03FFFFFF).contains(&pc) {
                (0x03800000, 0x800000)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        let is_thumb = match self.instruction_set {
            DisassemblerInstructionSet::Follow => match ARM_BOOL {
                ArmBool::ARM9 => emulator.arm9.cpsr.get_thumb(),
                ArmBool::ARM7 => emulator.arm7.cpsr.get_thumb(),
            },
            DisassemblerInstructionSet::Arm => false,
            DisassemblerInstructionSet::Thumb => true,
        };
        let instruction_width = if is_thumb { 2 } else { 4 };

        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        let total_rows = mem_size / instruction_width;
        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::remainder());

        if self.follow_pc {
            let pc_row = (pc - mem_start) / instruction_width;
            table_builder = table_builder.scroll_to_row(pc_row, Some(egui::Align::Center));
        };

        if let Some(jump_value) = self.jump_now {
            let jump_value = jump_value as usize;
            if jump_value >= mem_start && jump_value < mem_start + mem_size {
                let jump_row = (jump_value - mem_start) / instruction_width;
                table_builder = table_builder.scroll_to_row(jump_row, Some(egui::Align::Center));
            }
        };

        self.jump_now = None;

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
                    let mut address = mem_start + start;
                    if address >= mem_start + mem_size {
                        address += 0xFFFF0000;
                    };

                    let mem = match ARM_BOOL {
                        ArmBool::ARM9 => emulator.arm9.read_bulk(
                            &mut emulator.bus9,
                            &mut emulator.shared,
                            &mut emulator.dma9,
                            address as u32,
                            instruction_width as u32,
                        ),
                        ArmBool::ARM7 => emulator.arm7.read_bulk(
                            &mut emulator.bus7,
                            &mut emulator.shared,
                            &mut emulator.dma7,
                            address as u32,
                            instruction_width as u32,
                        ),
                    };

                    let inst = if is_thumb {
                        u16::from_le_bytes([mem[0], mem[1]]) as u32
                    } else {
                        u32::from_le_bytes([mem[0], mem[1], mem[2], mem[3]])
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
                                &mut fake_dma,
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
                                &mut fake_dma,
                                &mut disassembly,
                                &mut logger::FakeLogger,
                            ),
                        )
                    };
                    row.col(|ui| {
                        let is_breakpoint = self.breakpoints.contains(&(address as u32));
                        let text = format!("{:08X}", address);
                        let label = if is_breakpoint {
                            ui.colored_strong(egui::Color32::RED, text)
                        } else {
                            ui.label(text)
                        };
                        if label.clicked() {
                            ui.input(|r| {
                                if r.modifiers.ctrl {
                                    if is_breakpoint {
                                        self.breakpoints.retain(|&x| x != address as u32);
                                        self.selected_breakpoint = None;
                                    } else {
                                        self.breakpoints.push(address as u32);
                                        self.selected_breakpoint = Some(self.breakpoints.len() - 1);
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
                                            self.jump_now = Some(arg.raw);
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
