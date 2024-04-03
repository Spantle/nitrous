use crate::ui::{NitrousGUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_preferences(&mut self, ctx: &egui::Context) {
        let mut preferences = self.preferences;
        egui::Window::new_nitrous("Preferences", ctx)
            .open(&mut preferences)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.preferences_selected,
                        PreferencesPanel::Emulation,
                        "Emulation",
                    );
                });

                ui.separator();

                match self.preferences_selected {
                    PreferencesPanel::Emulation => {
                        self.show_emulation_preferences(ui);
                    }
                }
            });

        self.preferences = preferences;
    }

    fn show_emulation_preferences(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ARM9 BIOS file:");
            ui.text_edit_singleline(&mut self.preferences_arm9_bios_path);

            if ui.button("Browse").clicked() {
                let sender = self.load_arm9_bios_channel.0.clone();

                let task = rfd::AsyncFileDialog::new()
                    .add_filter("ARM9 BIOS", &["bin"])
                    .pick_file();

                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _result = sender.send(file.path().to_string_lossy().to_string());
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let _result = sender.send(file.read().await);
                        }

                        ctx.request_repaint();
                    }
                });
            }
        });
    }
}

#[derive(PartialEq)]
pub enum PreferencesPanel {
    Emulation,
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: core::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: core::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
