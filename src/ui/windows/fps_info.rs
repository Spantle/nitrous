use std::collections::VecDeque;

use web_time::{Duration, Instant};

use crate::ui::NitrousUI;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FpsInfoWindow {
    pub show: bool,

    #[serde(skip)]
    pub fps_counter: FpsCounter,
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

#[derive(Default)]
pub struct FpsCounter {
    last_frame_times: VecDeque<Instant>,
}

impl FpsCounter {
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

impl FpsInfoWindow {
    pub fn show(&mut self, ctx: &egui::Context, fps_info: FpsInfo) {
        if !self.show {
            return;
        }

        egui::Area::new(egui::Id::new("fps_info"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 32.0))
            .show(ctx, |ui| {
                let window_fill = ui.style().visuals.window_fill();
                let fill = egui::Color32::from_rgba_unmultiplied(
                    window_fill.r(),
                    window_fill.g(),
                    window_fill.b(),
                    150,
                );

                egui::Frame::popup(ui.style())
                    .shadow(egui::Shadow::NONE)
                    .fill(fill)
                    .show(ui, |ui| {
                        ui.make_monospace();
                        ui.set_min_width(250.0);

                        let table = egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .column(egui_extras::Column::exact(150.0))
                            .column(egui_extras::Column::remainder());

                        table.body(|mut body| {
                            row(
                                &mut body,
                                "Measured FPS",
                                &format!("{}", fps_info.measured_fps),
                            );
                            row(
                                &mut body,
                                "Emulation Frame Time",
                                &format!("{}ms", fps_info.emulation_time),
                            );
                            row(
                                &mut body,
                                "UI Last Frame Time",
                                &format!("{}ms", fps_info.last_ui_time),
                            );
                            row(
                                &mut body,
                                "Idle Last Frame Time",
                                &format!("{}ms", fps_info.last_idle_time),
                            );
                            row(
                                &mut body,
                                "ARM9 Last Cycles Ran",
                                &format!("{}", fps_info.last_cycles_ran_arm9),
                            );
                            row(
                                &mut body,
                                "ARM9 Target Cycles",
                                &format!("{}", fps_info.target_cycles_arm9),
                            );
                            row(
                                &mut body,
                                "ARM9 Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_arm9),
                            );
                            row(
                                &mut body,
                                "ARM7 Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_arm7),
                            );
                            row(
                                &mut body,
                                "GPU Cycles Ran",
                                &format!("{}", fps_info.cycles_ran_gpu),
                            );
                        });
                    });
            });
    }
}

fn row(body: &mut egui_extras::TableBody, label: &str, value: &str) {
    body.row(20.0, |mut row| {
        row.col(|ui| {
            ui.strong(label);
        });
        row.col(|ui| {
            ui.label(value);
        });
    });
}
