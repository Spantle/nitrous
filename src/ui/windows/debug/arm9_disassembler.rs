use crate::{
    nds::{
        cpu::{
            arm9::{self, models::Disassembly},
            bus::{self, BusTrait},
        },
        logger,
    },
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_arm9_disassembler(&mut self, ctx: &egui::Context) {
        let mut arm9_disassembler = self.arm9_disassembler;
        egui::Window::new_nitrous("ARM9 Disassembler", ctx)
            .open(&mut arm9_disassembler)
            .show(ctx, |ui| {
                self.render_instructions(ui);
            });

        self.arm9_disassembler = arm9_disassembler;
    }

    fn render_instructions(&mut self, ui: &mut egui::Ui) {
        ui.make_monospace();

        let bus = &self.emulator.bus;
        let mem = if bus.cart.arm9_size == 0 {
            vec![]
        } else {
            self.emulator
                .bus
                .read_bulk(bus.cart.arm9_load_address, bus.cart.arm9_size)
        };

        let pc = self.emulator.arm9.r[15] as usize;
        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        let total_rows = mem.len() / 4;
        let arm9_load_address = bus.cart.arm9_load_address as usize;
        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto());

        if self.emulator.is_running() {
            let pc_row = (pc - arm9_load_address) / 4;
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
                    let address = arm9_load_address + start;
                    let inst = u32::from_le_bytes([
                        mem[start],
                        mem[start + 1],
                        mem[start + 2],
                        mem[start + 3],
                    ]);

                    row.set_selected(address == pc);

                    let mut disassembly = Disassembly::default();
                    arm9::lookup_instruction_set(&mut arm9::models::Context::new(
                        inst.into(),
                        &mut arm9::FakeArm9::default(),
                        &mut bus::FakeBus,
                        &mut disassembly,
                        &mut logger::FakeLogger,
                    ));
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

fn match_color(kind: &arm9::models::ChunkKind) -> egui::Color32 {
    match kind {
        arm9::models::ChunkKind::Register => egui::Color32::from_rgb(190, 240, 250),
        arm9::models::ChunkKind::Immediate => egui::Color32::from_rgb(250, 90, 70),
        arm9::models::ChunkKind::Modifier => egui::Color32::from_rgb(210, 110, 210),
        arm9::models::ChunkKind::Punctuation => egui::Color32::from_rgb(140, 140, 140),
    }
}
