#[derive(PartialEq)]
pub enum ArmKind {
    Arm9,
    Arm7,
}

impl From<ArmKind> for bool {
    fn from(kind: ArmKind) -> bool {
        match kind {
            ArmKind::Arm9 => true,
            ArmKind::Arm7 => false,
        }
    }
}

pub struct ArmBool;
impl ArmBool {
    pub const ARM9: bool = true;
    pub const ARM7: bool = false;
}
