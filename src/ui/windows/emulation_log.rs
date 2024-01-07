use crate::{
    nds::logger,
    ui::{NitrousGUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_emulation_log(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("Emulation Log", ctx)
            .open(&mut self.emulation_log)
            .show(ctx, |ui| {
                let log = logger::LOGS.lock().unwrap();
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                let total_rows = log.len() / 16;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    total_rows,
                    |ui, row_range| {
                        for row in row_range {
                            let start = row * 16;
                            let end = start + 16;
                            let lines = &log[start..end];
                            for line in lines {
                                ui.label(&line.content);
                            }
                        }
                    },
                )
            });
    }
}
