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

        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        let total_rows = mem.len() / 4;
        let arm9_load_address = bus.cart.arm9_load_address as usize;
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
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

                    row.set_selected(address == self.emulator.arm9.r[15] as usize);

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
                        let disassembly: String = disassembly.into();
                        ui.label(disassembly);
                    });
                })
            })
    }
}
