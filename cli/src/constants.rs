pub const ADAPTERS: [&str; 2] = ["circom", "halo2"];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Adapter {
    Circom,
    Halo2,
}

impl Adapter {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl From<&str> for Adapter {
    fn from(adapter: &str) -> Self {
        match adapter.to_lowercase().as_str() {
            "circom" => Adapter::Circom,
            "halo2" => Adapter::Halo2,
            _ => panic!("Unknown adapter selected."),
        }
    }
}

impl From<usize> for Adapter {
    // `idx`` corresponds to the index of ADAPTERS array
    fn from(idx: usize) -> Self {
        match idx {
            0 => Self::Circom,
            1 => Self::Halo2,
            _ => unreachable!(),
        }
    }
}
