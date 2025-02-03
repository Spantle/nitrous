use std::time::{Duration, Instant};

use crate::{nds::Emulator, ui::NitrousWindow};

use super::arm::disassembler::ArmDisassemblerWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BenchmarkWindow {
    pub open: bool,
    pub cycles_to_run: u64,
    pub last_result: Option<Duration>,
}

impl Default for BenchmarkWindow {
    fn default() -> Self {
        Self {
            open: false,
            cycles_to_run: 66_000_000,
            last_result: None,
        }
    }
}

impl BenchmarkWindow {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        emulator: &mut Emulator,
        disassembler_windows: (&mut ArmDisassemblerWindow, &mut ArmDisassemblerWindow),
    ) {
        egui::Window::new_nitrous("Benchmark", ctx)
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let drag_value =
                        egui::DragValue::new(&mut self.cycles_to_run).speed(1_000_000.0);
                    ui.label("Cycles to run");
                    ui.add(drag_value);
                });
                ui.horizontal(|ui| {
                    let mut seconds = (self.cycles_to_run as f64) / 33_000_000_f64;
                    let drag_value = egui::DragValue::new(&mut seconds).speed(1_000_000.0);
                    ui.label("(Theoretical) Seconds to run");
                    ui.add(drag_value);

                    self.cycles_to_run = (seconds * 33_000_000_f64) as u64;
                });

                ui.horizontal(|ui| {
                    if ui.button("Run").clicked() {
                        let start_time = Instant::now();
                        emulator.start();
                        emulator.run_for(self.cycles_to_run, disassembler_windows);
                        emulator.pause();
                        let end_time = Instant::now();
                        self.last_result = Some(end_time - start_time);
                    }

                    if let Some(result) = self.last_result {
                        ui.label(format!("Last run took: {:?}", result));
                    }
                });
            });
    }
}
