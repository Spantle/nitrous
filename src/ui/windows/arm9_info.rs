use crate::{
    arm,
    ui::{NitrousGUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_arm9_info(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("ARM9 Info", ctx)
            .open(&mut self.arm9_info)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Register Values (Hexadecimal)")
                    .default_open(true)
                    .show(ui, |ui| {
                        let r = self.emulator.arm9.r;
                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .columns(egui_extras::Column::exact(60.0), r.len() + 1)
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Type");
                                });
                                for i in 0..r.len() {
                                    header.col(|ui| {
                                        match i {
                                            13 => {
                                                ui.strong("r13 (SP)").on_hover_text("Stack Pointer")
                                            }
                                            14 => {
                                                ui.strong("r14 (LR)").on_hover_text("Link Register")
                                            }
                                            15 => ui
                                                .strong("r15 (PC)")
                                                .on_hover_text("Program Counter"),
                                            _ => ui.strong(format!("r{}", i)),
                                        };
                                    });
                                }
                            })
                            .body(|mut body| {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.strong("Base");
                                    });
                                    for v in r {
                                        row.col(|ui| {
                                            ui.label(format!("{:08X}", v));
                                        });
                                    }
                                });

                                let mut display_range =
                                    |name: &str, min: usize, max: usize, registers: &[u32]| {
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.strong(name);
                                            });
                                            for i in 0..r.len() {
                                                if i < min || i > max {
                                                    row.col(|_ui| {});
                                                } else {
                                                    row.col(|ui| {
                                                        ui.label(format!(
                                                            "{:08X}",
                                                            registers[i - min]
                                                        ));
                                                    });
                                                }
                                            }
                                        });
                                    };

                                display_range("FIQ", 8, 14, &self.emulator.arm9.r_fiq);
                                display_range("IRQ", 13, 14, &self.emulator.arm9.r_irq);
                                display_range("SVC", 13, 14, &self.emulator.arm9.r_svc);
                                display_range("ABT", 13, 14, &self.emulator.arm9.r_abt);
                                display_range("UND", 13, 14, &self.emulator.arm9.r_und);
                            });
                    });

                egui::CollapsingHeader::new("Program Status Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(egui_extras::Column::exact(60.0))
                            .column(egui_extras::Column::exact(128.0))
                            .columns(egui_extras::Column::exact(60.0), 8)
                            .column(egui_extras::Column::exact(256.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Type");
                                });
                                header.col(|ui| {
                                    ui.strong("Mode");
                                });
                                header.col(|ui| {
                                    ui.strong("T").on_hover_text("Thumb");
                                });
                                header.col(|ui| {
                                    ui.strong("F").on_hover_text("FIQ Interrupt");
                                });
                                header.col(|ui| {
                                    ui.strong("I").on_hover_text("IRQ Interrupt");
                                });
                                header.col(|ui| {
                                    ui.strong("Q").on_hover_text("Saturation");
                                });
                                header.col(|ui| {
                                    ui.strong("V").on_hover_text("Overflow");
                                });
                                header.col(|ui| {
                                    ui.strong("C").on_hover_text("Carry");
                                });
                                header.col(|ui| {
                                    ui.strong("Z").on_hover_text("Zero");
                                });
                                header.col(|ui| {
                                    ui.strong("N").on_hover_text("Negative");
                                });
                                header.col(|ui| {
                                    ui.strong("Binary Value");
                                });
                            })
                            .body(|mut body| {
                                let mut display_psr = |name: &str, psr: &arm::PSR| {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            ui.strong(name);
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:?}", psr.mode()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.thumb()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.fiq_interrupt()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.irq_interrupt()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.saturation()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.overflow()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.carry()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.zero()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", psr.negative()));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:032b}", psr.value()));
                                        });
                                    });
                                };

                                display_psr("Base", &self.emulator.arm9.cpsr);
                                display_psr("FIQ", &self.emulator.arm9.r_fiq[7].into());
                                display_psr("IRQ", &self.emulator.arm9.r_irq[2].into());
                                display_psr("SVC", &self.emulator.arm9.r_svc[2].into());
                                display_psr("ABT", &self.emulator.arm9.r_abt[2].into());
                                display_psr("UND", &self.emulator.arm9.r_und[2].into());
                            });
                    });
            });
    }
}
