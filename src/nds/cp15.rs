#[derive(Debug)]
pub struct CP15 {
    pub inst_tcm: Vec<u8>, // 32kb
    pub data_tcm: Vec<u8>, // 16kb

    pub control_register: u32, // c1,c0,0

    pub data_tcm_base: u32,
    pub data_tcm_size: u32,
    pub inst_tcm_base: u32, // fixed/read-only
    pub inst_tcm_size: u32,
}

impl Default for CP15 {
    fn default() -> Self {
        Self {
            inst_tcm: vec![0; 1024 * 32],
            data_tcm: vec![0; 1024 * 16],

            control_register: 0b00000000000000000000000001111000,

            data_tcm_base: 0x00800000,
            data_tcm_size: 0x00040000,
            inst_tcm_base: 0x00000000,
            inst_tcm_size: 0x00008000,
        }
    }
}
