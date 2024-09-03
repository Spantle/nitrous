use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    nds::arm::{bus::BusTrait, ArmKind},
    ui::NitrousWindow,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PreferencesWindow {
    pub open: bool,
    #[serde(skip)]
    selected: PreferencesPanel,
    arm9_bios_path: String,
    arm7_bios_path: String,

    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    load_arm9_bios_channel: (Sender<String>, Receiver<String>),
    #[serde(skip)]
    #[cfg(target_arch = "wasm32")]
    load_arm9_bios_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    load_arm7_bios_channel: (Sender<String>, Receiver<String>),
    #[serde(skip)]
    #[cfg(target_arch = "wasm32")]
    load_arm7_bios_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
}

impl Default for PreferencesWindow {
    fn default() -> Self {
        Self {
            open: false,
            selected: PreferencesPanel::Emulation,
            arm9_bios_path: String::new(),
            arm7_bios_path: String::new(),

            load_arm9_bios_channel: channel(),
            load_arm7_bios_channel: channel(),
        }
    }
}

impl PreferencesWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        egui::Window::new_nitrous("Preferences", ctx)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.selected,
                        PreferencesPanel::Emulation,
                        "Emulation",
                    );
                });

                ui.separator();

                match self.selected {
                    PreferencesPanel::Emulation => {
                        self.show_emulation_preferences(ui);
                    }
                }
            });

        self.open = open;
    }

    fn show_emulation_preferences(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ARM9 BIOS file:");
            ui.text_edit_singleline(&mut self.arm9_bios_path);

            if ui.button("Browse").clicked() {
                let sender = self.load_arm9_bios_channel.0.clone();

                let task = rfd::AsyncFileDialog::new()
                    .add_filter("ARM9 BIOS", &["bin"])
                    .pick_file();

                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _result = sender.send(file.path().to_string_lossy().to_string());
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let _result = sender.send(file.read().await);
                        }

                        ctx.request_repaint();
                    }
                });
            }
        });

        ui.horizontal(|ui| {
            ui.label("ARM7 BIOS file:");
            ui.text_edit_singleline(&mut self.arm7_bios_path);

            if ui.button("Browse").clicked() {
                let sender = self.load_arm7_bios_channel.0.clone();

                let task = rfd::AsyncFileDialog::new()
                    .add_filter("ARM7 BIOS", &["bin"])
                    .pick_file();

                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _result = sender.send(file.path().to_string_lossy().to_string());
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let _result = sender.send(file.read().await);
                        }

                        ctx.request_repaint();
                    }
                });
            }
        });
    }

    pub fn try_load_bios<Bus: BusTrait>(&mut self, bus: &mut Bus) {
        let bios_path = match Bus::KIND {
            ArmKind::ARM9 => &self.arm9_bios_path,
            ArmKind::ARM7 => &self.arm7_bios_path,
        };
        if !bios_path.is_empty() {
            bus.load_bios_from_path(bios_path);
        }
    }

    pub fn load_bios_from_channel<Bus: BusTrait>(&mut self, bus: &mut Bus) {
        let (bios_channel, bios_path) = match Bus::KIND {
            ArmKind::ARM9 => (&self.load_arm9_bios_channel, &mut self.arm9_bios_path),
            ArmKind::ARM7 => (&self.load_arm7_bios_channel, &mut self.arm7_bios_path),
        };

        if let Ok(content) = bios_channel.1.try_recv() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                *bios_path = content;
                bus.load_bios_from_path(bios_path);
            }

            #[cfg(target_arch = "wasm32")]
            {
                bus.load_bios(content);
            }
        }
    }
}

#[derive(PartialEq)]
enum PreferencesPanel {
    Emulation,
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: core::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: core::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
