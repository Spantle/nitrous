use super::Bits;

pub struct CP15 {
    pub inst_tcm: Vec<u8>, // 32kb
    pub data_tcm: Vec<u8>, // 16kb

    pub control_register: Cp15ControlRegister, // c1,c0,0

    pub data_tcm_reg: u32,
    pub data_tcm_base: u32,
    pub data_tcm_size: u32,
    pub inst_tcm_reg: u32,
    pub inst_tcm_base: u32, // fixed/read-only
    pub inst_tcm_size: u32,
}

impl Default for CP15 {
    fn default() -> Self {
        Self {
            inst_tcm: vec![0; 1024 * 32],
            data_tcm: vec![0; 1024 * 16],

            control_register: Cp15ControlRegister(0b00000000000000000000000001111000),

            data_tcm_reg: 0x00000000,
            data_tcm_base: 0x00800000,
            data_tcm_size: 0x00040000,
            inst_tcm_reg: 0x00000000,
            inst_tcm_base: 0x00000000,
            inst_tcm_size: 0x00008000,
        }
    }
}

pub struct Cp15ControlRegister(u32);

impl From<u32> for Cp15ControlRegister {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Cp15ControlRegister {
    const DATA_TCM_LOAD_MODE_OFFSET: u32 = 17;
    const INSTRUCTION_TCM_LOAD_MODE_OFFSET: u32 = 19;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn get_data_tcm_load_mode(&self) -> bool {
        self.0.get_bit(Self::DATA_TCM_LOAD_MODE_OFFSET)
    }

    pub fn get_instruction_tcm_load_mode(&self) -> bool {
        self.0.get_bit(Self::INSTRUCTION_TCM_LOAD_MODE_OFFSET)
    }
}
