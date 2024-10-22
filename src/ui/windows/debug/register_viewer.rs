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
            "DISPSTAT",
            "VCOUNT",
            "[A]DISPCNT",
            "[A]BG0CNT",
            "[A]BG1CNT",
            "[A]BG2CNT",
            "[A]BG3CNT",
            "[B]DISPCNT",
            "[B]BG0CNT",
            "[B]BG1CNT",
            "[B]BG2CNT",
            "[B]BG3CNT",
            "KEYINPUT",
            "EXTKEYIN",
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
            "AUXSPICNT",
            "COMMAND",
            "EXMEMCNT",
            "EXMEMSTAT",
            "ROMCTRL",
        ]
        .into()
    }

    fn values(&self, emulator: &Emulator) -> Vec<usize> {
        [
            emulator.shared.gpus.dispstat.value() as usize,
            emulator.shared.gpus.vcount as usize,
            emulator.shared.gpus.a.dispcnt.value() as usize,
            emulator.shared.gpus.a.bgxcnt[0].value() as usize,
            emulator.shared.gpus.a.bgxcnt[1].value() as usize,
            emulator.shared.gpus.a.bgxcnt[2].value() as usize,
            emulator.shared.gpus.a.bgxcnt[3].value() as usize,
            emulator.shared.gpus.b.dispcnt.value() as usize,
            emulator.shared.gpus.b.bgxcnt[0].value() as usize,
            emulator.shared.gpus.b.bgxcnt[1].value() as usize,
            emulator.shared.gpus.b.bgxcnt[2].value() as usize,
            emulator.shared.gpus.b.bgxcnt[3].value() as usize,
            emulator.shared.keyinput.value() as usize,
            emulator.shared.extkeyin.value() as usize,
            emulator.shared.vramcnt[0] as usize,
            emulator.shared.vramcnt[1] as usize,
            emulator.shared.vramcnt[2] as usize,
            emulator.shared.vramcnt[3] as usize,
            emulator.shared.vramcnt[4] as usize,
            emulator.shared.vramcnt[5] as usize,
            emulator.shared.vramcnt[6] as usize,
            emulator.shared.vramcnt[7] as usize,
            emulator.shared.vramcnt[8] as usize,
            emulator.shared.vramcnt[9] as usize,
            emulator.shared.ipcsync.value_quiet::<true>() as usize,
            emulator.shared.ipcsync.value_quiet::<false>() as usize,
            emulator.shared.ipcfifo.get_cnt::<true>() as usize,
            emulator.shared.ipcfifo.get_cnt::<false>() as usize,
            emulator.bus9.interrupts.me.value() as usize,
            emulator.bus9.interrupts.e.value() as usize,
            emulator.bus9.interrupts.f.value() as usize,
            emulator.bus7.interrupts.me.value() as usize,
            emulator.bus7.interrupts.e.value() as usize,
            emulator.bus7.interrupts.f.value() as usize,
            emulator.shared.powcnt1.value() as usize,
            emulator.arm9.cp15.control_register.value() as usize,
            emulator.arm9.cp15.data_tcm_reg as usize,
            emulator.arm9.cp15.inst_tcm_reg as usize,
            emulator.shared.cart.auxspicnt.value() as usize,
            emulator.shared.cart.command.0 as usize,
            emulator.shared.cart.exmemcnt.0 as usize,
            emulator.shared.cart.exmemstat.0 as usize,
            emulator.shared.cart.romctrl.value() as usize,
        ]
        .into()
    }
}
