use crate::ui::{NitrousGUI, NitrousUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_register_viewer(&mut self, ctx: &egui::Context) {
        let mut register_viewer = self.register_viewer;
        egui::Window::new_nitrous("Register Viewer", ctx)
            .open(&mut register_viewer)
            .show(ctx, |ui| {
                ui.make_monospace();

                self.render_values(ui);
            });

        self.register_viewer = register_viewer;
    }

    fn render_values(&mut self, ui: &mut egui::Ui) {
        let names = self.names();
        let values = self.values();

        let column_count = 2;
        let column_width = 80.0;
        let row_height = 20.0;
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(egui_extras::Column::exact(column_width), column_count)
            .header(row_height, |mut header| {
                header.col(|c| {
                    c.label("Register");
                });
                header.col(|c| {
                    c.label("Value");
                });
            })
            .body(|mut body| {
                for i in 0..names.len() {
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(names[i]);
                        });
                        row.col(|ui| {
                            ui.label(format!("{:08X}", values[i]));
                        });
                    });
                }
            })
    }

    fn names(&self) -> [&str; 15] {
        [
            "DISPCNT",
            "DISPSTAT",
            "VCOUNT",
            "KEYINPUT",
            "VRAMCNT_A",
            "VRAMCNT_B",
            "VRAMCNT_C",
            "VRAMCNT_D",
            "VRAMCNT_E",
            "VRAMCNT_F",
            "VRAMCNT_G",
            "WRAMCNT",
            "VRAMCNT_H",
            "VRAMCNT_I",
            "POWCNT1",
        ]
    }

    fn values(&self) -> [u32; 15] {
        [
            self.emulator.shared.gpu2d_a.dispcnt.value(),
            self.emulator.shared.gpu2d_a.dispstat.value() as u32,
            self.emulator.shared.gpu2d_a.vcount as u32,
            self.emulator.shared.keyinput.value() as u32,
            self.emulator.shared.vramcnt[0] as u32,
            self.emulator.shared.vramcnt[1] as u32,
            self.emulator.shared.vramcnt[2] as u32,
            self.emulator.shared.vramcnt[3] as u32,
            self.emulator.shared.vramcnt[4] as u32,
            self.emulator.shared.vramcnt[5] as u32,
            self.emulator.shared.vramcnt[6] as u32,
            self.emulator.shared.vramcnt[7] as u32,
            self.emulator.shared.vramcnt[8] as u32,
            self.emulator.shared.vramcnt[9] as u32,
            self.emulator.shared.powcnt1.value(),
        ]
    }
}
