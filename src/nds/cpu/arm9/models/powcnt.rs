#[derive(Default)]
pub struct POWCNT1(u32);

impl From<u32> for POWCNT1 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl POWCNT1 {
    const BOTH_LCDS_ENABLED_OFFSET: u32 = 0;
    const ENG_2D_A_OFFSET: u32 = 1;
    const ENG_3D_RENDERING_OFFSET: u32 = 2;
    const ENG_3D_GEOMETRY_OFFSET: u32 = 3;
    const ENG_2D_B_OFFSET: u32 = 9;
    const DISPLAY_SWAP_OFFSET: u32 = 15;

    pub fn value(&self) -> u32 {
        self.0
    }

    fn get_bit(&self, offset: u32) -> bool {
        (self.0 >> offset) & 1 == 1
    }

    fn set_bit(&mut self, offset: u32, value: bool) {
        self.0 = (self.0 & !(1 << offset)) | ((value as u32) << offset);
    }

    pub fn get_both_lcds_enabled(&self) -> bool {
        self.get_bit(Self::BOTH_LCDS_ENABLED_OFFSET)
    }

    pub fn set_both_lcds_enabled(&mut self, both_lcds_enabled: bool) {
        self.set_bit(Self::BOTH_LCDS_ENABLED_OFFSET, both_lcds_enabled);
    }

    pub fn get_eng_2d_a(&self) -> bool {
        self.get_bit(Self::ENG_2D_A_OFFSET)
    }

    pub fn set_eng_2d_a(&mut self, eng_2d_a: bool) {
        self.set_bit(Self::ENG_2D_A_OFFSET, eng_2d_a);
    }

    pub fn get_eng_3d_rendering(&self) -> bool {
        self.get_bit(Self::ENG_3D_RENDERING_OFFSET)
    }

    pub fn set_eng_3d_rendering(&mut self, eng_3d_rendering: bool) {
        self.set_bit(Self::ENG_3D_RENDERING_OFFSET, eng_3d_rendering);
    }

    pub fn get_eng_3d_geometry(&self) -> bool {
        self.get_bit(Self::ENG_3D_GEOMETRY_OFFSET)
    }

    pub fn set_eng_3d_geometry(&mut self, eng_3d_geometry: bool) {
        self.set_bit(Self::ENG_3D_GEOMETRY_OFFSET, eng_3d_geometry);
    }

    pub fn get_eng_2d_b(&self) -> bool {
        self.get_bit(Self::ENG_2D_B_OFFSET)
    }

    pub fn set_eng_2d_b(&mut self, eng_2d_b: bool) {
        self.set_bit(Self::ENG_2D_B_OFFSET, eng_2d_b);
    }

    pub fn get_display_swap(&self) -> bool {
        self.get_bit(Self::DISPLAY_SWAP_OFFSET)
    }

    pub fn set_display_swap(&mut self, display_swap: bool) {
        self.set_bit(Self::DISPLAY_SWAP_OFFSET, display_swap);
    }
}
