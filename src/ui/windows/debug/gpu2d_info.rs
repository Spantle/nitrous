use crate::ui::{NitrousGUI, NitrousUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_gpu2d_info(&mut self, ctx: &egui::Context) {
        let mut gpu2d_info = self.gpu2d_info;
        egui::Window::new_nitrous("2D GPU Info", ctx)
            .open(&mut gpu2d_info)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Register Values (Hexadecimal)")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.make_monospace();

                        self.register_values(ui);
                    })
            });

        self.gpu2d_info = gpu2d_info;
    }

    fn register_values(&mut self, ui: &mut egui::Ui) {
        let column_count = 2;
        let column_width = 60.0;
        let row_height = 20.0;
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(egui_extras::Column::exact(column_width), column_count)
            .header(row_height, |mut header| {
                header.col(|_| ());
                header.col(|c| {
                    c.label("Value");
                });
            })
            .body(|mut body| {
                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.strong("DISPCNT");
                    });
                    row.col(|ui| {
                        ui.label(format!("{:08X}", self.emulator.bus.gpu2d_a.dispcnt.value()));
                    });
                });
            })
    }
}
