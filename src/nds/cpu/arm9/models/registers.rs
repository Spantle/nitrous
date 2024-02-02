#[derive(Clone, Copy, Debug)]
pub struct Registers(pub [u32; 16]);

impl std::ops::Index<u8> for Registers {
    type Output = u32;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl std::ops::Deref for Registers {
    type Target = [u32; 16]; // Specify the target type to dereference to

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Registers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Registers {
    type Item = u32;
    type IntoIter = std::array::IntoIter<Self::Item, 16>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::ops::IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
