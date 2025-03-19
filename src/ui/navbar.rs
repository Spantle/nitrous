use std::io::{Read, Write};

use crate::nds::logger;

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

        if ui.button("Save state").clicked() {
            let result = serde_json::to_string(&self.emulator);
            match result {
                Ok(state) => {
                    let task = rfd::AsyncFileDialog::new()
                        .add_filter("Nitrous State", &["gz"])
                        .save_file();

                    let ctx = ui.ctx().clone();
                    execute(async move {
                        let file = task.await;
                        if let Some(file) = file {
                            // compress with zstd_safe
                            let mut e = flate2::write::GzEncoder::new(
                                Vec::new(),
                                flate2::Compression::default(),
                            );
                            let success = e.write_all(state.as_bytes());
                            match success {
                                Ok(_) => {
                                    let compressed = e.finish();
                                    match compressed {
                                        Ok(compressed) => {
                                            let result = file.write(compressed.as_slice()).await;
                                            match result {
                                                Ok(_) => {
                                                    logger::info(
                                                        logger::LogSource::Emu,
                                                        "Emulator state saved",
                                                    );
                                                }
                                                Err(e) => {
                                                    logger::error(
                                                        logger::LogSource::Emu,
                                                        format!(
                                                            "Failed to save emulator state: {}",
                                                            e
                                                        ),
                                                    );
                                                }
                                            }
                                            ctx.request_repaint();
                                        }
                                        Err(e) => {
                                            logger::error(
                                                logger::LogSource::Emu,
                                                format!("Failed to compress emulator state: {}", e),
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    logger::error(
                                        logger::LogSource::Emu,
                                        format!("Failed to compress emulator state: {}", e),
                                    );
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    logger::error(
                        logger::LogSource::Emu,
                        format!("Failed to serialize emulator state: {}", e),
                    );
                }
            }

            return true;
        }

        if ui.button("Load state").clicked() {
            let channel = self.load_state_channel.0.clone();
            let task = rfd::AsyncFileDialog::new()
                .add_filter("Nitrous State", &["gz"])
                .pick_file();

            let ctx = ui.ctx().clone();
            execute(async move {
                let file = task.await;
                if let Some(file) = file {
                    let bytes = file.read().await;
                    let mut d = flate2::read::GzDecoder::new(bytes.as_slice());
                    let mut decompressed_bytes = Vec::new();
                    d.read_to_end(&mut decompressed_bytes).unwrap();

                    let result = serde_json::from_slice(&decompressed_bytes);
                    match result {
                        Ok(emulator) => {
                            logger::info(logger::LogSource::Emu, "Loading emulator state");
                            let _result = channel.send(emulator);
                            ctx.request_repaint();
                        }
                        Err(e) => {
                            logger::error(
                                logger::LogSource::Emu,
                                format!("Failed to load emulator state: {}", e),
                            );
                        }
                    }
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
            self.emulator.reset(true);
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

            ui.checkbox(&mut self.gpu_map_viewer.open, "Map Viewer (WIP)");
            ui.checkbox(&mut self.gpu_palette_viewer.open, "Palette Viewer (WIP)");
            ui.checkbox(&mut self.gpu_tile_viewer.open, "Tile Viewer (WIP)");

            ui.menu_button("[A] Toggle BGs", |ui| {
                let bgs = &mut self.emulator.shared.gpus.a.show_bgs;
                ui.checkbox(&mut bgs[0], "BG 0");
                ui.checkbox(&mut bgs[1], "BG 1");
                ui.checkbox(&mut bgs[2], "BG 2");
                ui.checkbox(&mut bgs[3], "BG 3");
            });
            ui.menu_button("[A] Toggle OBJs", |ui| {
                let objs = &mut self.emulator.shared.gpus.a.show_objs;
                ui.checkbox(&mut objs[0], "OBJs 0");
                ui.checkbox(&mut objs[1], "OBJs 1");
                ui.checkbox(&mut objs[2], "OBJs 2");
                ui.checkbox(&mut objs[3], "OBJs 3");
            });
            ui.menu_button("[B] Toggle BGs", |ui| {
                let bgs = &mut self.emulator.shared.gpus.b.show_bgs;
                ui.checkbox(&mut bgs[0], "BG 0");
                ui.checkbox(&mut bgs[1], "BG 1");
                ui.checkbox(&mut bgs[2], "BG 2");
                ui.checkbox(&mut bgs[3], "BG 3");
            });
            ui.menu_button("[B] Toggle OBJs", |ui| {
                let objs = &mut self.emulator.shared.gpus.b.show_objs;
                ui.checkbox(&mut objs[0], "OBJs 0");
                ui.checkbox(&mut objs[1], "OBJs 1");
                ui.checkbox(&mut objs[2], "OBJs 2");
                ui.checkbox(&mut objs[3], "OBJs 3");
            });
        });
        ui.checkbox(&mut self.benchmark.open, "Benchmark");
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
