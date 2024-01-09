use super::{NitrousGUI, NitrousUI};

impl NitrousGUI {
    pub fn show_navbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Organize").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }

                ui.menu_button("Emulation", |ui| {
                    let close = Self::emulation_menu(self, ui);

                    if close {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Debug", |ui| {
                    let close = Self::debug_menu(self, ui);

                    if close {
                        debug!("Closing debug menu");
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn emulation_menu(&mut self, ui: &mut egui::Ui) -> bool {
        let running = self.emulator.is_running();
        let started = ui.enabled_button(!running, "Start emulation", || self.emulator.start());
        let paused = ui.enabled_button(running, "Pause emulation", || self.emulator.pause());

        started || paused
    }

    fn debug_menu(&mut self, ui: &mut egui::Ui) -> bool {
        ui.checkbox(&mut self.arm9_info, "ARM9 Info");
        ui.checkbox(&mut self.emulation_log, "Emulation Log");
        ui.checkbox(&mut self.memory_viewer, "Memory Viewer");
        ui.checkbox(&mut self.test_window, "Test Window");

        if ui.button("Test button").clicked() {
            debug!("Test button clicked");
            return true;
        }

        false
    }
}
