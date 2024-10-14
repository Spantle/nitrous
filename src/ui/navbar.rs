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

                ui.menu_button("Screens", |ui| {
                    let close = Self::screens_menu(self, ui);

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
                        .selectable_label(self.fps_info.show, format!("FPS: {}", estimated_fps))
                        .clicked()
                    {
                        self.fps_info.show = !self.fps_info.show;
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

        ui.checkbox(&mut self.preferences.open, "Preferences");

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
            self.emulator.step();
        }

        started || paused || reset
    }

    fn screens_menu(&mut self, ui: &mut egui::Ui) -> bool {
        if ui.button("Add top screen").clicked() {
            self.screen_options
                .top_screens
                .push(self.screen_options.top_screen_count);
            self.screen_options.top_screen_count += 1;
        };
        if ui.button("Add bottom screen").clicked() {
            self.screen_options
                .bot_screens
                .push(self.screen_options.bot_screen_count);
            self.screen_options.bot_screen_count += 1;
        };
        if ui.button("Add dual screen").clicked() {
            self.screen_options
                .duo_screens
                .push(self.screen_options.duo_screen_count);
            self.screen_options.duo_screen_count += 1;
        };
        if ui.button("Remove all screens").clicked() {
            self.screen_options.top_screens.clear();
            self.screen_options.bot_screens.clear();
            self.screen_options.duo_screens.clear();
            self.screen_options.top_screen_count = 0;
            self.screen_options.bot_screen_count = 0;
            self.screen_options.duo_screen_count = 0;
        };

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.screen_options.scale, 0.0..=8.0));
        });
        ui.checkbox(&mut self.screen_options.fit, "Fit to window");
        ui.menu_button("Horizontal alignment", |ui| {
            ui.set_width(100.0);

            ui.selectable_value(
                &mut self.screen_options.horizontal_alignment,
                egui::Align::LEFT,
                "Left",
            );
            ui.selectable_value(
                &mut self.screen_options.horizontal_alignment,
                egui::Align::Center,
                "Center",
            );
            ui.selectable_value(
                &mut self.screen_options.horizontal_alignment,
                egui::Align::RIGHT,
                "Right",
            );
        });

        false
    }

    fn debug_menu(&mut self, ui: &mut egui::Ui) -> bool {
        ui.menu_button("ARM9", |ui| {
            ui.set_width(150.0);

            ui.checkbox(&mut self.arm9_disassembler.open, "ARM9 Disassembler");
            ui.checkbox(&mut self.arm9_info.open, "ARM9 Info");
            ui.checkbox(&mut self.arm9_info_legacy.open, "(Legacy) ARM9 Info");
        });
        ui.menu_button("ARM7", |ui| {
            ui.set_width(150.0);

            ui.checkbox(&mut self.arm7_disassembler.open, "ARM7 Disassembler");
            ui.checkbox(&mut self.arm7_info.open, "ARM7 Info");
        });
        ui.menu_button("GPU", |ui| {
            ui.set_width(150.0);

            ui.checkbox(&mut self.gpu_palette_viewer.open, "Palette Viewer");
        });
        ui.checkbox(&mut self.emulation_log.open, "Emulation Log");
        ui.checkbox(&mut self.ipcsync_log.open, "IPCSYNC Log");
        ui.checkbox(&mut self.memory_viewer.open, "Memory Viewer");
        ui.checkbox(&mut self.register_viewer.open, "Register Viewer");
        ui.checkbox(&mut self.test_window.open, "Test Window");

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
