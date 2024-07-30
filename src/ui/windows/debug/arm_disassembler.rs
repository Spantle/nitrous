use crate::{
    nds::{
        arm::{self, bus, models::Disassembly, ArmBool},
        logger, shared,
    },
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

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
                self.render_instructions::<ARM_BOOL>(ui);
            });

        match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_disassembler = arm_disassembler,
            ArmBool::ARM7 => self.arm7_disassembler = arm_disassembler,
        };
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

        let pc = match ARM_BOOL {
            ArmBool::ARM9 => self.emulator.arm9.r[15],
            ArmBool::ARM7 => self.emulator.arm7.r[15],
        } as usize;
        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        let total_rows = mem.len() / 4;
        let arm_load_address = arm_load_address as usize;
        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto());

        if self.emulator.is_running() {
            if pc < arm_load_address || pc >= arm_load_address + arm_size as usize {
                return;
            }
            let pc_row = (pc - arm_load_address) / 4;
            table_builder = table_builder.scroll_to_row(pc_row, None);
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
                    let start = row.index() * 4;
                    let address = arm_load_address + start;
                    let inst = u32::from_le_bytes([
                        mem[start],
                        mem[start + 1],
                        mem[start + 2],
                        mem[start + 3],
                    ]);

                    row.set_selected(address == pc);

                    let mut disassembly = Disassembly::default();
                    arm::lookup_instruction_set::<{ ArmBool::ARM9 }>(
                        &mut arm::models::Context::new(
                            inst.into(),
                            &mut arm::FakeArm::new(address as u32),
                            &mut fake_bus,
                            &mut fake_shared,
                            &mut disassembly,
                            &mut logger::FakeLogger,
                        ),
                    );
                    row.col(|ui| {
                        ui.label(format!("{:08X}", address));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:08X}", inst));
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
