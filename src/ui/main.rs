use std::sync::mpsc::{channel, Receiver, Sender};

use egui::load::SizedTexture;
use web_time::{Duration, Instant};

use crate::nds::{
    cpu::arm::{bus::BusTrait, ArmBool},
    logger, Emulator,
};

use super::windows::file::preferences::PreferencesPanel;

#[cfg(not(target_arch = "wasm32"))]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(egui::vec2(272.0, 427.0)),
        ..Default::default()
    };
    eframe::run_native("Nitrous", options, Box::new(|cc| Ok(creator(cc, emulator))))
}

#[cfg(target_arch = "wasm32")]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("nitrous", options, Box::new(|cc| Ok(creator(cc, emulator))))
            .await
            .expect("Failed to run web app"); // TODO: can we pass this error out somehow?
    });

    Ok(())
}

fn creator(cc: &eframe::CreationContext, emulator: Emulator) -> Box<dyn eframe::App> {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    cc.egui_ctx.set_fonts(fonts);

    Box::new(NitrousGUI::new(cc, emulator))
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NitrousGUI {
    #[serde(skip)]
    is_first_run: bool,

    #[serde(skip)]
    pub emulator: Emulator,

    #[serde(skip)]
    pub load_rom_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    pub load_arm9_bios_channel: (Sender<String>, Receiver<String>),
    #[serde(skip)]
    #[cfg(target_arch = "wasm32")]
    pub load_arm9_bios_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),

    // Debug
    pub arm9_disassembler: bool,
    pub arm9_info: bool,
    pub arm9_info_legacy: bool,

    pub arm7_disassembler: bool,
    pub arm7_info: bool,

    pub emulation_log: bool,
    pub ipcsync_log: bool,
    pub memory_viewer: bool,
    pub register_viewer: bool,
    pub test_window: bool,

    // File
    pub preferences: bool,

    #[serde(skip)]
    pub arm9_info_legacy_selected: Option<(String, usize)>,
    #[serde(skip)]
    pub arm9_info_legacy_selected_pending_value: String,

    #[serde(skip)]
    pub memory_viewer_selected: Option<usize>,
    #[serde(skip)]
    pub memory_viewer_selected_pending_value: Option<u8>,

    #[serde(skip)]
    pub preferences_selected: PreferencesPanel,
    pub preferences_arm9_bios_path: String,

    #[serde(skip)]
    fps_outliers: u8,
    #[serde(skip)]
    idle_times: [u128; 60],
    #[serde(skip)]
    ui_times: [u128; 60],
    #[serde(skip)]
    last_ui_time: Duration,
    #[serde(skip)]
    last_frame_end: Instant,
}

impl Default for NitrousGUI {
    fn default() -> Self {
        NitrousGUI {
            is_first_run: true,

            emulator: Emulator::default(),

            load_rom_channel: channel(),
            load_arm9_bios_channel: channel(),

            arm9_disassembler: false,
            arm9_info: false,
            arm9_info_legacy: false,

            arm7_disassembler: false,
            arm7_info: false,

            emulation_log: false,
            ipcsync_log: false,
            memory_viewer: false,
            register_viewer: false,
            test_window: false,

            preferences: false,

            arm9_info_legacy_selected: None,
            arm9_info_legacy_selected_pending_value: String::new(),

            memory_viewer_selected: None,
            memory_viewer_selected_pending_value: None,

            preferences_selected: PreferencesPanel::Emulation,
            preferences_arm9_bios_path: String::new(),

            fps_outliers: 0,
            idle_times: [0; 60],
            ui_times: [0; 60],
            last_ui_time: Duration::default(),
            last_frame_end: Instant::now(),
        }
    }
}

impl NitrousGUI {
    pub fn new(cc: &eframe::CreationContext<'_>, emulator: Emulator) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        NitrousGUI {
            emulator,

            ..Default::default()
        }
    }
}

impl eframe::App for NitrousGUI {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let idle_time = self.last_frame_end.elapsed();

        // logger::debug(
        //     logger::LogSource::Emu,
        //     format!(
        //         "{:?} : {:?}",
        //         idle_time.as_millis(),
        //         self.last_ui_time.as_millis()
        //     ),
        // );

        self.handle_input(ctx);

        let start_ui = Instant::now();

        let idle_time_sum = self.idle_times.iter().sum::<u128>();
        let frame_time_sum = self.ui_times.iter().sum::<u128>() + idle_time_sum;
        let estimated_compute_time = idle_time_sum / 60;
        let estimated_fps = 1_000_000 / (frame_time_sum / 60).max(1);
        let max_cycles = 66_000_000 / estimated_fps;

        // logger::debug(
        //     logger::LogSource::Emu,
        //     format!(
        //         "Estimated Compute Time: {} Estimated FPS: {} Max Cycles: {}",
        //         estimated_compute_time, estimated_fps, max_cycles
        //     ),
        // );

        if self.emulator.is_running() {
            let start_time = Instant::now();
            for _ in 0..max_cycles {
                if !self.emulator.is_running() {
                    break;
                }

                if start_time.elapsed().as_micros() > estimated_compute_time {
                    break;
                }

                self.emulator.clock();
            }
        }

        let idle_micros = idle_time.as_micros();
        // (estimated_compute_time / 1000) != 0 can be removed, it's just to reduce debug logs
        if (estimated_compute_time / 1000) != 0 && idle_micros > estimated_compute_time * 10 {
            logger::debug(
                logger::LogSource::Emu,
                format!(
                    "FPS Outlier detected ({}ms/{}ms)",
                    idle_micros / 1000,
                    estimated_compute_time / 1000
                ),
            );

            if self.fps_outliers >= 3 {
                logger::debug(logger::LogSource::Emu, "Too many outliers".to_string());
                self.idle_times.rotate_left(1);
                self.idle_times[59] = idle_micros;
            } else {
                self.fps_outliers += 1;
            }
        } else {
            self.idle_times.rotate_left(1);
            self.idle_times[59] = idle_micros;
            self.fps_outliers = 0;
        }

        let ui_micros = self.last_ui_time.as_micros();
        self.ui_times.rotate_left(1);
        self.ui_times[59] = ui_micros;

        self.show_navbar(
            ctx,
            FpsInfo {
                estimated_compute_time,
                estimated_fps,
                max_cycles,
                idle_time: idle_micros,
                ui_time: ui_micros,
            },
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::StripBuilder::new(ui)
                .size(egui_extras::Size::exact(192.0))
                .size(egui_extras::Size::exact(192.0))
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(egui_extras::Size::exact(256.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let image =
                                        self.emulator.shared.gpu2d_a.render(&self.emulator.shared);
                                    let texture = ui.ctx().load_texture(
                                        "top_screen",
                                        image,
                                        Default::default(),
                                    );
                                    ui.add(egui::Image::from_texture(SizedTexture::from_handle(
                                        &texture,
                                    )));
                                });
                            });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(egui_extras::Size::exact(256.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let image =
                                        self.emulator.shared.gpu2d_b.render(&self.emulator.shared);
                                    let texture = ui.ctx().load_texture(
                                        "bottom_screen",
                                        image,
                                        Default::default(),
                                    );
                                    ui.add(egui::Image::from_texture(SizedTexture::from_handle(
                                        &texture,
                                    )));
                                });
                            });
                    })
                });
        });

        // Debug
        self.show_arm_disassembler::<{ ArmBool::ARM9 }>(ctx);
        self.show_arm_disassembler::<{ ArmBool::ARM7 }>(ctx);
        self.show_arm_info::<{ ArmBool::ARM9 }>(ctx);
        self.show_arm_info::<{ ArmBool::ARM7 }>(ctx);
        self.show_arm9_info_legacy(ctx);
        self.show_emulation_log(ctx);
        self.show_ipcsync_log(ctx);
        self.show_memory_viewer(ctx);
        self.show_register_viewer(ctx);
        self.show_test_window(ctx);

        // File
        self.show_preferences(ctx);

        if !self.emulator.is_running() {
            // do slow stuff if idle
            if self.is_first_run {
                self.is_first_run = false;

                if !self.preferences_arm9_bios_path.is_empty() {
                    self.emulator
                        .bus9
                        .load_bios_from_path(&self.preferences_arm9_bios_path);
                }
            }

            if let Ok(bytes) = self.load_rom_channel.1.try_recv() {
                self.emulator.load_rom(bytes);
            }

            if let Ok(content) = self.load_arm9_bios_channel.1.try_recv() {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.preferences_arm9_bios_path = content;
                    self.emulator
                        .bus9
                        .load_bios_from_path(&self.preferences_arm9_bios_path);
                }

                #[cfg(target_arch = "wasm32")]
                {
                    self.emulator.bus9.load_bios(content);
                }
            }
        }

        self.last_ui_time = start_ui.elapsed();
        self.last_frame_end = Instant::now();

        ctx.request_repaint();
    }
}

pub struct FpsInfo {
    pub estimated_compute_time: u128,
    pub estimated_fps: u128,
    pub max_cycles: u128,
    pub idle_time: u128,
    pub ui_time: u128,
}
