use crate::{
    nds::cpu::arm::{models::PSR, ArmBool},
    ui::{NitrousGUI, NitrousUI, NitrousWindow},
};

impl NitrousGUI {
    pub fn show_arm_info<const ARM_BOOL: bool>(&mut self, ctx: &egui::Context) {
        let mut arm_info = match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_info,
            ArmBool::ARM7 => self.arm7_info,
        };
        let title = match ARM_BOOL {
            ArmBool::ARM9 => "ARM9 Info",
            ArmBool::ARM7 => "ARM7 Info",
        };

        egui::Window::new_nitrous(title, ctx)
            .open(&mut arm_info)
            .show(ctx, |ui| {
                self.render_arm_info::<ARM_BOOL>(ui);
            });

        match ARM_BOOL {
            ArmBool::ARM9 => self.arm9_info = arm_info,
            ArmBool::ARM7 => self.arm7_info = arm_info,
        };
    }

    fn render_arm_info<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        ui.make_monospace();

        egui::CollapsingHeader::new("Register Values (Hex)")
            .default_open(true)
            .show(ui, |ui| {
                self.render_reg_values::<ARM_BOOL>(ui);
            });

        egui::CollapsingHeader::new("Program Status Register Values")
            .default_open(true)
            .show(ui, |ui| {
                self.render_psrs::<ARM_BOOL>(ui);
            });

        egui::CollapsingHeader::new("Program Status Registers (Binary)")
            .default_open(false)
            .show(ui, |ui| {
                self.render_psr_values::<ARM_BOOL>(ui);
            });
    }

    fn render_reg_values<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        let (r, r_fiq, r_irq, r_svc, r_abt, r_und) = match ARM_BOOL {
            ArmBool::ARM9 => (
                &self.emulator.arm9.r,
                &self.emulator.arm9.r_fiq,
                &self.emulator.arm9.r_irq,
                &self.emulator.arm9.r_svc,
                &self.emulator.arm9.r_abt,
                &self.emulator.arm9.r_und,
            ),
            ArmBool::ARM7 => (
                &self.emulator.arm7.r,
                &self.emulator.arm7.r_fiq,
                &self.emulator.arm7.r_irq,
                &self.emulator.arm7.r_svc,
                &self.emulator.arm7.r_abt,
                &self.emulator.arm7.r_und,
            ),
        };

        let table = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(egui_extras::Column::exact(60.0), 7);

        table
            .header(20.0, |mut header| {
                header.col(|_ui| {});
                header.col(|ui| {
                    ui.strong("Base");
                });
                header.col(|ui| {
                    ui.strong("FIQ");
                });
                header.col(|ui| {
                    ui.strong("IRQ");
                });
                header.col(|ui| {
                    ui.strong("SVC");
                });
                header.col(|ui| {
                    ui.strong("ABT");
                });
                header.col(|ui| {
                    ui.strong("UND");
                });
            })
            .body(|mut body| {
                for i in 0..r.len() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            match i {
                                13 => ui.strong("r13 (SP)").on_hover_text("Stack Pointer"),
                                14 => ui.strong("r14 (LR)").on_hover_text("Link Register"),
                                15 => ui.strong("r15 (PC)").on_hover_text("Program Counter"),
                                _ => ui.strong(format!("r{}", i)),
                            };
                        });

                        row.col(|ui| {
                            ui.label(format!("{:08X}", r[i as u8]));
                        });

                        row.col(|ui| {
                            if (8..=14).contains(&i) {
                                ui.label(format!("{:08X}", r_fiq[i - 8]));
                            };
                        });

                        if (13..=14).contains(&i) {
                            row.col(|ui| {
                                ui.label(format!("{:08X}", r_irq[i - 13]));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:08X}", r_svc[i - 13]));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:08X}", r_abt[i - 13]));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:08X}", r_und[i - 13]));
                            });
                        } else {
                            for _ in 0..4 {
                                row.col(|_ui| {});
                            }
                        }
                    });
                }
            });
    }

    fn render_psrs<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        let (cpsr, psr_fiq, psr_irq, psr_svc, psr_abt, psr_und) = match ARM_BOOL {
            ArmBool::ARM9 => (
                &self.emulator.arm9.cpsr,
                PSR::from(self.emulator.arm9.r_fiq[7]),
                PSR::from(self.emulator.arm9.r_irq[2]),
                PSR::from(self.emulator.arm9.r_svc[2]),
                PSR::from(self.emulator.arm9.r_abt[2]),
                PSR::from(self.emulator.arm9.r_und[2]),
            ),
            ArmBool::ARM7 => (
                &self.emulator.arm7.cpsr,
                PSR::from(self.emulator.arm7.r_fiq[7]),
                PSR::from(self.emulator.arm7.r_irq[2]),
                PSR::from(self.emulator.arm7.r_svc[2]),
                PSR::from(self.emulator.arm7.r_abt[2]),
                PSR::from(self.emulator.arm7.r_und[2]),
            ),
        };

        let table = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .columns(egui_extras::Column::exact(60.0), 7);

        table
            .header(20.0, |mut header| {
                header.col(|_ui| {});
                header.col(|ui| {
                    ui.strong("Base");
                });
                header.col(|ui| {
                    ui.strong("FIQ");
                });
                header.col(|ui| {
                    ui.strong("IRQ");
                });
                header.col(|ui| {
                    ui.strong("SVC");
                });
                header.col(|ui| {
                    ui.strong("ABT");
                });
                header.col(|ui| {
                    ui.strong("UND");
                });
            })
            .body(|mut body| {
                for i in 0..=8 {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            match i {
                                0 => ui.strong("Mode"),
                                1 => ui.strong("T").on_hover_text("Thumb"),
                                2 => ui.strong("F").on_hover_text("FIQ Interrupt"),
                                3 => ui.strong("I").on_hover_text("IRQ Interrupt"),
                                4 => ui.strong("Q").on_hover_text("Saturation"),
                                5 => ui.strong("V").on_hover_text("Overflow"),
                                6 => ui.strong("C").on_hover_text("Carry"),
                                7 => ui.strong("Z").on_hover_text("Zero"),
                                8 => ui.strong("N").on_hover_text("Negative"),
                                _ => unreachable!(),
                            };
                        });

                        if i == 0 {
                            let col = |row: &mut egui_extras::TableRow, psr: &PSR| {
                                row.col(|ui| {
                                    ui.label(psr.get_mode().to_string());
                                });
                            };

                            col(&mut row, cpsr);
                            col(&mut row, &psr_fiq);
                            col(&mut row, &psr_irq);
                            col(&mut row, &psr_svc);
                            col(&mut row, &psr_abt);
                            col(&mut row, &psr_und);
                        } else {
                            let getter = |psr: &PSR| match i {
                                1 => psr.get_thumb(),
                                2 => psr.get_fiq_interrupt(),
                                3 => psr.get_irq_interrupt(),
                                4 => psr.get_saturation(),
                                5 => psr.get_overflow(),
                                6 => psr.get_carry(),
                                7 => psr.get_zero(),
                                8 => psr.get_negative(),
                                _ => unreachable!(),
                            };
                            let col = |row: &mut egui_extras::TableRow, psr: &PSR| {
                                row.col(|ui| {
                                    let mut v = getter(psr);
                                    ui.checkbox(&mut v, "");
                                });
                            };

                            col(&mut row, cpsr);
                            col(&mut row, &psr_fiq);
                            col(&mut row, &psr_irq);
                            col(&mut row, &psr_svc);
                            col(&mut row, &psr_abt);
                            col(&mut row, &psr_und);
                        }
                    });
                }
            });
    }

    fn render_psr_values<const ARM_BOOL: bool>(&mut self, ui: &mut egui::Ui) {
        let (cpsr, psr_fiq, psr_irq, psr_svc, psr_abt, psr_und) = match ARM_BOOL {
            ArmBool::ARM9 => (
                self.emulator.arm9.cpsr.value(),
                PSR::from(self.emulator.arm9.r_fiq[7]).value(),
                PSR::from(self.emulator.arm9.r_irq[2]).value(),
                PSR::from(self.emulator.arm9.r_svc[2]).value(),
                PSR::from(self.emulator.arm9.r_abt[2]).value(),
                PSR::from(self.emulator.arm9.r_und[2]).value(),
            ),
            ArmBool::ARM7 => (
                self.emulator.arm7.cpsr.value(),
                PSR::from(self.emulator.arm7.r_fiq[7]).value(),
                PSR::from(self.emulator.arm7.r_irq[2]).value(),
                PSR::from(self.emulator.arm7.r_svc[2]).value(),
                PSR::from(self.emulator.arm7.r_abt[2]).value(),
                PSR::from(self.emulator.arm7.r_und[2]).value(),
            ),
        };

        let table = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::exact(60.0))
            .column(egui_extras::Column::exact(400.0));

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Mode");
                });
                header.col(|ui| {
                    ui.strong("Value");
                });
            })
            .body(|mut body| {
                for i in 0..=5 {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            match i {
                                0 => ui.strong("Base"),
                                1 => ui.strong("FIQ"),
                                2 => ui.strong("IRQ"),
                                3 => ui.strong("SVC"),
                                4 => ui.strong("ABT"),
                                5 => ui.strong("UND"),
                                _ => unreachable!(),
                            };
                        });

                        let value = match i {
                            0 => cpsr,
                            1 => psr_fiq,
                            2 => psr_irq,
                            3 => psr_svc,
                            4 => psr_abt,
                            5 => psr_und,
                            _ => unreachable!(),
                        };

                        row.col(|ui| {
                            ui.label(format!("{:032b}", value));
                        });
                    });
                }
            });
    }
}
