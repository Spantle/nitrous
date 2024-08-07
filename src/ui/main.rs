use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
};

use egui::load::SizedTexture;
use web_time::{Duration, Instant};

use crate::nds::{
    arm::{bus::BusTrait, ArmBool},
    Emulator,
};

use super::windows::{self, file::preferences::PreferencesPanel};

// this whole file needs a good clean

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
    pub arm9_disassembler_instruction_set:
        windows::debug::arm_disassembler::DisassemblerInstructionSet,
    pub arm9_disassembler_follow_pc: bool,
    #[serde(skip)]
    pub arm9_disassembler_jump_value: String,
    #[serde(skip)]
    pub arm9_disassembler_jump_now: bool,
    #[serde(skip)]
    pub arm7_disassembler_instruction_set:
        windows::debug::arm_disassembler::DisassemblerInstructionSet,
    pub arm7_disassembler_follow_pc: bool,
    #[serde(skip)]
    pub arm7_disassembler_jump_value: String,
    #[serde(skip)]
    pub arm7_disassembler_jump_now: bool,

    #[serde(skip)]
    pub memory_viewer_selected: Option<usize>,
    #[serde(skip)]
    pub memory_viewer_selected_pending_value: Option<u8>,

    #[serde(skip)]
    pub preferences_selected: PreferencesPanel,
    pub preferences_arm9_bios_path: String,

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

            arm9_disassembler_instruction_set:
                windows::debug::arm_disassembler::DisassemblerInstructionSet::Follow,
            arm9_disassembler_follow_pc: true,
            arm9_disassembler_jump_value: String::new(),
            arm9_disassembler_jump_now: false,
            arm7_disassembler_instruction_set:
                windows::debug::arm_disassembler::DisassemblerInstructionSet::Follow,
            arm7_disassembler_follow_pc: true,
            arm7_disassembler_jump_value: String::new(),
            arm7_disassembler_jump_now: false,

            memory_viewer_selected: None,
            memory_viewer_selected_pending_value: None,

            preferences_selected: PreferencesPanel::Emulation,
            preferences_arm9_bios_path: String::new(),

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

        let mut cycles_ran_arm9 = 0;
        let mut cycles_ran_arm7 = self.last_cycle_arm7_discrepency as u64;
        let mut cycles_ran_gpu = 0;

        if self.emulator.is_running() {
            while cycles_ran_arm9 < target_cycles_arm9 {
                if !self.emulator.is_running() {
                    break;
                }

                let arm9_cycles = self
                    .emulator
                    .arm9
                    .clock(&mut self.emulator.bus9, &mut self.emulator.shared);

                cycles_ran_arm9 += arm9_cycles as u64;

                let target_cycles_arm7 = cycles_ran_arm9 / 2;
                let target_cycles_gpu = cycles_ran_arm9 / 2;

                while cycles_ran_arm7 < target_cycles_arm7 {
                    let arm7_cycles = self
                        .emulator
                        .arm7
                        .clock(&mut self.emulator.bus7, &mut self.emulator.shared);
                    cycles_ran_arm7 += arm7_cycles as u64;
                }

                while cycles_ran_gpu < target_cycles_gpu {
                    self.emulator.shared.gpu2d_a.clock();
                    self.emulator.shared.gpu2d_b.clock();
                    cycles_ran_gpu += 1;
                }
            }
        }

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
