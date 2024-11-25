#[derive(Clone, Copy, Debug, Default)]
pub enum Mst {
    #[default]
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}

impl From<u8> for Mst {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum Offset {
    #[default]
    A,
    B,
    C,
    D,
}

impl From<u8> for Offset {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            _ => unreachable!(),
        }
    }
}
