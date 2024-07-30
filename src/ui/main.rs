use std::sync::mpsc::{channel, Receiver, Sender};

use egui::load::SizedTexture;
use web_time::{Duration, Instant};

use crate::nds::{
    arm::{bus::BusTrait, ArmBool},
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

    pub fps_info: bool,

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
    emulation_times: [u128; 60],
    #[serde(skip)]
    ui_times: [u128; 60],
    #[serde(skip)]
    last_time_ui: Duration,
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

            fps_info: false,

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
            emulation_times: [0; 60],
            ui_times: [0; 60],
            last_time_ui: Duration::default(),
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
        let last_time_idle = self.last_frame_end.elapsed();

        self.handle_input(ctx);

        let start_time_emulation = Instant::now();

        let time_sum_idle = self.idle_times.iter().sum::<u128>();
        let time_sum_emulation = self.emulation_times.iter().sum::<u128>();
        let time_sum_total =
            self.ui_times.iter().sum::<u128>() + time_sum_idle + time_sum_emulation;
        let time_micros_total = time_sum_total / 60;
        let target_emulation_time =
            ((((time_sum_idle / 60) + (time_sum_emulation / 60)) as f64) * 0.75).round() as u128;
        let estimated_fps = (1_000_000 / time_micros_total.max(1)) as u32;
        let target_cycles_per_frame = 99_000_000 / estimated_fps;
        let mut cycles_run = 0;

        if self.emulator.is_running() {
            let start_time = Instant::now();
            while cycles_run < target_cycles_per_frame {
                if !self.emulator.is_running() {
                    break;
                }

                if start_time.elapsed().as_micros() >= target_emulation_time {
                    break;
                }

                let cycles = self.emulator.clock();

                cycles_run += cycles;
            }
        }

        let last_micros_idle = last_time_idle.as_micros();
        // (estimated_compute_time / 1000) != 0 can be removed, it's just to reduce debug logs
        if (target_emulation_time / 1000) != 0 && last_micros_idle > target_emulation_time * 10 {
            logger::debug(
                logger::LogSource::Emu,
                format!(
                    "FPS Outlier detected ({}ms/{}ms)",
                    last_micros_idle / 1000,
                    target_emulation_time / 1000
                ),
            );

            if self.fps_outliers >= 3 {
                logger::debug(logger::LogSource::Emu, "Too many outliers".to_string());
                self.idle_times.rotate_left(1);
                self.idle_times[59] = last_micros_idle;
            } else {
                self.fps_outliers += 1;
            }
        } else {
            self.idle_times.rotate_left(1);
            self.idle_times[59] = last_micros_idle;
            self.fps_outliers = 0;
        }

        let last_time_emulation = start_time_emulation.elapsed();
        let last_micros_emulation = last_time_emulation.as_micros();
        self.emulation_times.rotate_left(1);
        self.emulation_times[59] = last_micros_emulation;

        let start_time_ui = Instant::now();

        let last_micros_ui = self.last_time_ui.as_micros();
        self.ui_times.rotate_left(1);
        self.ui_times[59] = last_micros_ui;

        self.show_navbar(ctx, estimated_fps);

        self.show_fps_info(
            ctx,
            FpsInfo {
                estimated_fps,
                target_emulation_time,
                target_cycles_per_frame,
                cycles_run,
                last_micros_idle,
                last_micros_emulation,
                last_micros_ui,
                total_micros_frame: time_micros_total,
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

        self.last_time_ui = start_time_ui.elapsed();
        self.last_frame_end = Instant::now();

        ctx.request_repaint();
    }
}

pub struct FpsInfo {
    pub estimated_fps: u32,
    pub target_emulation_time: u128,
    pub target_cycles_per_frame: u32,
    pub cycles_run: u32,
    pub last_micros_idle: u128,
    pub last_micros_emulation: u128,
    pub last_micros_ui: u128,
    pub total_micros_frame: u128,
}
