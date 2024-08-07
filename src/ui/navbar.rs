use super::{NitrousGUI, NitrousUI};

impl NitrousGUI {
    pub fn show_navbar(&mut self, ctx: &egui::Context, estimated_fps: u32) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    ui.set_width(150.0);

                    let close = Self::file_menu(self, ui);

                    if close {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Emulation", |ui| {
                    ui.set_width(150.0);

                    let close = Self::emulation_menu(self, ui);

                    if close {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Debug", |ui| {
                    ui.set_width(150.0);

                    let close = Self::debug_menu(self, ui);

                    if close {
                        debug!("Closing debug menu");
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .selectable_label(self.fps_info, format!("FPS: {}", estimated_fps))
                        .clicked()
                    {
                        self.fps_info = !self.fps_info;
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

        if ui.button("Organize windows").clicked() {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        false
    }

    fn emulation_menu(&mut self, ui: &mut egui::Ui) -> bool {
        let running = self.emulator.is_running();
        let started = ui.enabled_button(!running, "Start emulation", || self.emulator.start());
        let paused = ui.enabled_button(running, "Pause emulation", || self.emulator.pause());
        let reset = if ui.button("Reset emulator").clicked() {
            self.emulator.reset();
            true
        } else {
            false
        };
        if ui.button("Step emulation").clicked() {
            self.emulator.clock();
        }

        started || paused || reset
    }

    fn debug_menu(&mut self, ui: &mut egui::Ui) -> bool {
        ui.menu_button("ARM9", |ui| {
            ui.set_width(150.0);

            ui.checkbox(&mut self.arm9_disassembler, "ARM9 Disassembler");
            ui.checkbox(&mut self.arm9_info, "ARM9 Info");
            ui.checkbox(&mut self.arm9_info_legacy, "(Legacy) ARM9 Info");
        });
        ui.menu_button("ARM7", |ui| {
            ui.set_width(150.0);

            ui.checkbox(&mut self.arm7_disassembler, "ARM7 Disassembler");
            ui.checkbox(&mut self.arm7_info, "ARM7 Info");
        });
        ui.checkbox(&mut self.emulation_log, "Emulation Log");
        ui.checkbox(&mut self.ipcsync_log, "IPCSYNC Log");
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
