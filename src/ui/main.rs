use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
};

use egui::load::SizedTexture;
use web_time::{Duration, Instant};

use crate::nds::{arm::ArmBool, Emulator};

use super::windows::{
    debug::{
        arm::{disassembler::ArmDisassemblerWindow, info::ArmInfoWindow},
        arm9_info::Arm9LegacyInfoWindow,
        emulation_log::EmulationLogWindow,
        ipcsync::IpcsyncLogWindow,
        memory_viewer::MemoryViewerWindow,
        register_viewer::RegisterViewerWindow,
        test_window::TestWindow,
    },
    file::preferences::PreferencesWindow,
};

// this whole file needs a good clean

#[cfg(not(target_arch = "wasm32"))]
pub fn init(emulator: Emulator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(egui::vec2(272.0, 427.0)),
        ..Default::default()
    };
    eframe::run_native(
        "NitrousDS",
        options,
        Box::new(|cc| Ok(creator(cc, emulator))),
    )
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

    pub fps_info: bool,

    // Debug Windows
    pub arm9_disassembler: ArmDisassemblerWindow,
    pub arm9_info: ArmInfoWindow,
    pub arm9_info_legacy: Arm9LegacyInfoWindow,

    pub arm7_disassembler: ArmDisassemblerWindow,
    pub arm7_info: ArmInfoWindow,

    pub emulation_log: EmulationLogWindow,
    pub ipcsync_log: IpcsyncLogWindow,
    pub memory_viewer: MemoryViewerWindow,
    pub register_viewer: RegisterViewerWindow,
    pub test_window: TestWindow,

    // File Windows
    pub preferences: PreferencesWindow,

    #[serde(skip)]
    fps_counter: FpsCounter,
    #[serde(skip)]
    last_cycle_count: u64,
    #[serde(skip)]
    last_frame_cycles_execution_time: Duration,
    #[serde(skip)]
    last_cycle_arm7_discrepency: i32,

    #[serde(skip)]
    last_end_instant: Instant,
    #[serde(skip)]
    last_ui_time: Duration,
}

impl Default for NitrousGUI {
    fn default() -> Self {
        NitrousGUI {
            is_first_run: true,

            emulator: Emulator::default(),

            load_rom_channel: channel(),

            fps_info: false,

            arm9_disassembler: ArmDisassemblerWindow::default(),
            arm9_info: ArmInfoWindow::default(),
            arm9_info_legacy: Arm9LegacyInfoWindow::default(),

            arm7_disassembler: ArmDisassemblerWindow::default(),
            arm7_info: ArmInfoWindow::default(),

            emulation_log: EmulationLogWindow::default(),
            ipcsync_log: IpcsyncLogWindow::default(),
            memory_viewer: MemoryViewerWindow::default(),
            register_viewer: RegisterViewerWindow::default(),
            test_window: TestWindow::default(),

            preferences: PreferencesWindow::default(),

            fps_counter: FpsCounter::new(),
            last_cycle_count: 0,
            last_frame_cycles_execution_time: Duration::ZERO,
            last_cycle_arm7_discrepency: 0,

            last_end_instant: Instant::now(),
            last_ui_time: Duration::ZERO,
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
        let idle_time = self.last_end_instant.elapsed();

        let emulation_start_time = Instant::now();

        self.handle_input(ctx);

        self.fps_counter.push_current_time();
        let measured_fps = self.fps_counter.average_fps();

        let target_frame_time_secs = 1.0 / measured_fps as f64;
        let target_frame_cycle_execution_time_secs = target_frame_time_secs * 0.75;

        let measured_fps_min_capped = measured_fps.max(10.0); // For when the window is minimized, to recover easier
        let mut target_cycles_current_frame = (66_000_000.0 / measured_fps_min_capped) as u64;

        if self.last_frame_cycles_execution_time != Duration::ZERO && self.last_cycle_count != 0 {
            let measured_nanoseconds_per_instruction =
                self.last_frame_cycles_execution_time.as_nanos() as f64
                    / self.last_cycle_count as f64;

            let target_frame_cycle_execution_time_nanos =
                target_frame_cycle_execution_time_secs * 1_000_000_000.0;
            let max_cycles_allowed_for_target_frame_time =
                target_frame_cycle_execution_time_nanos / measured_nanoseconds_per_instruction;

            target_cycles_current_frame =
                target_cycles_current_frame.min(max_cycles_allowed_for_target_frame_time as u64);
        }

        let frame_cycles_start = Instant::now();

        let target_cycles_arm9 = target_cycles_current_frame;

        let (cycles_ran_arm9, cycles_ran_arm7, cycles_ran_gpu) = self.emulator.run_for(
            target_cycles_arm9,
            self.last_cycle_arm7_discrepency as u64,
            (&mut self.arm9_disassembler, &mut self.arm7_disassembler),
        );

        let arm7_discrepency = (cycles_ran_arm7 as i32) - (cycles_ran_arm9 / 2) as i32;
        self.last_cycle_arm7_discrepency = arm7_discrepency;

        self.last_cycle_count = cycles_ran_arm9;
        self.last_frame_cycles_execution_time = frame_cycles_start.elapsed();

        let emulation_time = emulation_start_time.elapsed();

        let ui_start_time = Instant::now();

        self.show_navbar(ctx, measured_fps as u32);

        self.show_fps_info(
            ctx,
            FpsInfo {
                measured_fps: measured_fps as u32,
                emulation_time: emulation_time.as_millis() as u32,
                last_ui_time: self.last_ui_time.as_millis() as u32,
                last_idle_time: idle_time.as_millis() as u32,
                target_cycles_arm9,
                cycles_ran_arm9,
                cycles_ran_arm7,
                cycles_ran_gpu,
                last_cycles_ran_arm9: self.last_cycle_count,
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
        self.arm9_disassembler
            .show::<{ ArmBool::ARM9 }>(&mut self.emulator, ctx);
        self.arm9_info
            .show::<{ ArmBool::ARM9 }>(&mut self.emulator, ctx);
        self.arm9_info_legacy.show(&mut self.emulator, ctx);

        self.arm7_disassembler
            .show::<{ ArmBool::ARM7 }>(&mut self.emulator, ctx);
        self.arm7_info
            .show::<{ ArmBool::ARM7 }>(&mut self.emulator, ctx);

        self.emulation_log.show(ctx);
        self.ipcsync_log.show(&mut self.emulator, ctx);
        self.memory_viewer.show(&mut self.emulator, ctx);
        self.register_viewer.show(&self.emulator, ctx);
        self.test_window.show(ctx);

        // File
        self.preferences.show(ctx);

        if !self.emulator.is_running() {
            // do slow stuff if idle
            if self.is_first_run {
                self.is_first_run = false;

                self.preferences.try_load_bios(&mut self.emulator.bus9);
                self.preferences.try_load_bios(&mut self.emulator.bus7);
            }

            if let Ok(bytes) = self.load_rom_channel.1.try_recv() {
                self.emulator.load_rom(bytes);
            }

            self.preferences
                .load_bios_from_channel(&mut self.emulator.bus9);
            self.preferences
                .load_bios_from_channel(&mut self.emulator.bus7);
        }

        ctx.request_repaint();

        self.last_ui_time = ui_start_time.elapsed();
        self.last_end_instant = Instant::now();
    }
}

pub struct FpsInfo {
    pub measured_fps: u32,
    pub emulation_time: u32,
    pub last_ui_time: u32,
    pub last_idle_time: u32,
    pub last_cycles_ran_arm9: u64,
    pub target_cycles_arm9: u64,
    pub cycles_ran_arm9: u64,
    pub cycles_ran_arm7: u64,
    pub cycles_ran_gpu: u64,
}

struct FpsCounter {
    last_frame_times: VecDeque<Instant>,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            last_frame_times: VecDeque::new(),
        }
    }

    pub fn push_current_time(&mut self) {
        // Strip times that are over 500ms old, always keep the last one
        let now = Instant::now();
        while self.last_frame_times.len() > 1 {
            let second_last_time = self.last_frame_times[1];
            if now.duration_since(second_last_time) > Duration::from_millis(500) {
                self.last_frame_times.pop_front();
            } else {
                break;
            }
        }

        // Push current time
        self.last_frame_times.push_back(now);
    }

    pub fn average_fps(&self) -> f32 {
        // Calculate the average duration between pairs
        let mut total_duration = Duration::ZERO;
        for i in 1..self.last_frame_times.len() {
            total_duration += self.last_frame_times[i].duration_since(self.last_frame_times[i - 1]);
        }

        let frame_time = total_duration.as_secs_f32() / (self.last_frame_times.len() - 1) as f32;
        1.0 / frame_time
    }
}

// struct RollingAverageTime {
//     past_times: VecDeque<Duration>,
// }

// impl RollingAverageTime {
//     pub fn new() -> Self {
//         RollingAverageTime {
//             past_times: VecDeque::new(),
//         }
//     }

//     pub fn push_time(&mut self, time: Duration) {
//         self.past_times.push_back(time);

//         while self.past_times.len() > 60 {
//             self.past_times.pop_front();
//         }
//     }

//     pub fn average_time(&self) -> Duration {
//         let mut total_time = Duration::ZERO;
//         for time in &self.past_times {
//             total_time += *time;
//         }

//         total_time / self.past_times.len() as u32
//     }
// }
