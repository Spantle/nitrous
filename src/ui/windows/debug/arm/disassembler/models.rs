use std::fmt::Display;

use crate::nds::arm;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ArmDisassemblerWindow {
    pub open: bool,

    #[serde(skip)]
    pub instruction_set: DisassemblerInstructionSet,
    pub follow_pc: bool,
    #[serde(skip)]
    pub jump_value: String,
    #[serde(skip)]
    pub jump_now: Option<u32>,
    #[serde(skip)]
    pub step_amount: String,
    #[serde(skip)]
    pub step_until: Option<u32>,
    pub breakpoints: Vec<u32>,
    pub selected_breakpoint: Option<usize>,
}

impl Default for ArmDisassemblerWindow {
    fn default() -> Self {
        Self {
            open: false,
            instruction_set: DisassemblerInstructionSet::Follow,
            follow_pc: true,
            jump_value: String::new(),
            jump_now: None,
            step_amount: "1".to_string(),
            step_until: None,
            breakpoints: Vec::new(),
            selected_breakpoint: None,
        }
    }
}

#[derive(PartialEq)]
pub enum DisassemblerInstructionSet {
    Follow,
    Arm,
    Thumb,
}

impl Display for DisassemblerInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisassemblerInstructionSet::Follow => write!(f, "Follow CPU"),
            DisassemblerInstructionSet::Arm => write!(f, "ARM"),
            DisassemblerInstructionSet::Thumb => write!(f, "THUMB"),
        }
    }
}

pub fn match_color(kind: &arm::models::ChunkKind) -> egui::Color32 {
    match kind {
        arm::models::ChunkKind::Register => egui::Color32::from_rgb(190, 240, 250),
        arm::models::ChunkKind::Immediate => egui::Color32::from_rgb(250, 90, 70),
        arm::models::ChunkKind::Modifier => egui::Color32::from_rgb(210, 110, 210),
        arm::models::ChunkKind::Punctuation => egui::Color32::from_rgb(140, 140, 140),
    }
}
