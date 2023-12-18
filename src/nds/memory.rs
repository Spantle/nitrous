pub trait Memory {
    fn read_u32(&self, addr: u32) -> u32;
}

impl Memory for Vec<u8> {
    fn read_u32(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self[addr..addr + 4]);
        u32::from_le_bytes(bytes)
    }
}
