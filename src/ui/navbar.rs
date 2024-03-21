use super::{NitrousGUI, NitrousUI};

impl NitrousGUI {
    pub fn show_navbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Organize").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }

                ui.menu_button("File", |ui| {
                    let close = Self::file_menu(self, ui);

                    if close {
                        ui.close_menu();
                    }
                });

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

    fn file_menu(&mut self, ui: &mut egui::Ui) -> bool {
        if ui.button("Open ROM").clicked() {
            let sender = self.load_rom_channel.0.clone();

            let task = rfd::AsyncFileDialog::new()
                .add_filter("NDS ROM", &["nds"])
                .pick_file();

            let ctx = ui.ctx().clone();
            execute(async move {
                let file = task.await;
                if let Some(file) = file {
                    let bytes = file.read().await;
                    let _result = sender.send(bytes);
                    ctx.request_repaint();
                }
            });
            return true;
        }

        ui.separator();

        ui.checkbox(&mut self.preferences, "Preferences");

        false
    }

    fn emulation_menu(&mut self, ui: &mut egui::Ui) -> bool {
        let running = self.emulator.is_running();
        let started = ui.enabled_button(!running, "Start emulation", || self.emulator.start());
        let paused = ui.enabled_button(running, "Pause emulation", || self.emulator.pause());
        if ui.button("Step emulation").clicked() {
            self.emulator.step();
        }

        started || paused
    }

    fn debug_menu(&mut self, ui: &mut egui::Ui) -> bool {
        ui.checkbox(&mut self.arm9_disassembler, "ARM9 Disassembler");
        ui.checkbox(&mut self.arm9_info, "ARM9 Info");
        ui.checkbox(&mut self.emulation_log, "Emulation Log");
        ui.checkbox(&mut self.memory_viewer, "Memory Viewer");
        ui.checkbox(&mut self.register_viewer, "Register Viewer");
        ui.checkbox(&mut self.test_window, "Test Window");

        if ui.button("Test button").clicked() {
            debug!("Test button clicked");
            return true;
        }

        false
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: core::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: core::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
