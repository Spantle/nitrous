use crate::{
    nds::Emulator,
    ui::{NitrousUI, NitrousWindow},
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RegisterViewerWindow {
    pub open: bool,
}

impl RegisterViewerWindow {
    pub fn show(&mut self, emulator: &Emulator, ctx: &egui::Context) {
        let mut open = self.open;
        egui::Window::new_nitrous("Register Viewer", ctx)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.make_monospace();

                self.render_values(emulator, ui);
            });

        self.open = open;
    }

    fn render_values(&mut self, emulator: &Emulator, ui: &mut egui::Ui) {
        let names = self.names();
        let values = self.values(emulator);

        let column_count = 2;
        let column_width = 80.0;
        let row_height = 20.0;
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(egui_extras::Column::exact(column_width), column_count)
            .header(row_height, |mut header| {
                header.col(|c| {
                    c.label("Register");
                });
                header.col(|c| {
                    c.label("Value");
                });
            })
            .body(|mut body| {
                for i in 0..names.len() {
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(names[i]);
                        });
                        row.col(|ui| {
                            ui.label(format!("{:08X}", values[i]));
                        });
                    });
                }
            })
    }

    fn names(&self) -> Vec<&str> {
        [
            "[A]DISPCNT",
            "[A]DISPSTAT",
            "[A]VCOUNT",
            "[B]DISPCNT",
            "[B]DISPSTAT",
            "[B]VCOUNT",
            "KEYINPUT",
            "VRAMCNT_A",
            "VRAMCNT_B",
            "VRAMCNT_C",
            "VRAMCNT_D",
            "VRAMCNT_E",
            "VRAMCNT_F",
            "VRAMCNT_G",
            "WRAMCNT",
            "VRAMCNT_H",
            "VRAMCNT_I",
            "IPCSYNC9",
            "IPCSYNC7",
            "IPCFIFO_CNT9",
            "IPCFIFO_CNT7",
            "IME9",
            "IE9",
            "IF9",
            "IME7",
            "IE7",
            "IF7",
            "POWCNT1",
            "CP15_CNT",
            "CP15_DTCM",
            "CP15_ITCM",
        ]
        .into()
    }

    fn values(&self, emulator: &Emulator) -> Vec<u32> {
        [
            emulator.shared.gpu2d_a.dispcnt.value(),
            emulator.shared.gpu2d_a.dispstat.value() as u32,
            emulator.shared.gpu2d_a.vcount as u32,
            emulator.shared.gpu2d_b.dispcnt.value(),
            emulator.shared.gpu2d_b.dispstat.value() as u32,
            emulator.shared.gpu2d_b.vcount as u32,
            emulator.shared.keyinput.value() as u32,
            emulator.shared.vramcnt[0] as u32,
            emulator.shared.vramcnt[1] as u32,
            emulator.shared.vramcnt[2] as u32,
            emulator.shared.vramcnt[3] as u32,
            emulator.shared.vramcnt[4] as u32,
            emulator.shared.vramcnt[5] as u32,
            emulator.shared.vramcnt[6] as u32,
            emulator.shared.vramcnt[7] as u32,
            emulator.shared.vramcnt[8] as u32,
            emulator.shared.vramcnt[9] as u32,
            emulator.shared.ipcsync.value_quiet::<true>(),
            emulator.shared.ipcsync.value_quiet::<false>(),
            emulator.shared.ipcfifo.get_cnt::<true>(),
            emulator.shared.ipcfifo.get_cnt::<false>(),
            emulator.bus9.interrupts.me.value(),
            emulator.bus9.interrupts.e.value(),
            emulator.bus9.interrupts.f.value(),
            emulator.bus7.interrupts.me.value(),
            emulator.bus7.interrupts.e.value(),
            emulator.bus7.interrupts.f.value(),
            emulator.shared.powcnt1.value(),
            emulator.arm9.cp15.control_register.value(),
            emulator.arm9.cp15.data_tcm_reg,
            emulator.arm9.cp15.inst_tcm_reg,
        ]
        .into()
    }
}
