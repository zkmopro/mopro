use crate::{
    constants::{Adapter, ADAPTERS},
    select::multi_select,
};

pub fn contains_circom(path: &str) -> bool {
    path.to_lowercase().contains("circom")
}

pub fn contains_halo2(path: &str) -> bool {
    path.to_lowercase().contains("halo2")
}

pub struct AdapterSelector {
    adapters: Vec<Adapter>,
}

impl AdapterSelector {
    pub fn construct(selections: Vec<usize>) -> Self {
        let mut adapters: Vec<Adapter> = vec![];
        for s in selections {
            adapters.push(ADAPTERS[s].into());
        }
        Self { adapters }
    }

    pub fn select() -> Self {
        let adapters = multi_select(
            "Pick the adapters you want to use (multiple selection with space)",
            "No adapters selected. Use space to select an adapter",
            ADAPTERS.to_vec(),
        );

        Self {
            adapters: adapters.iter().map(|&p| p.into()).collect::<Vec<Adapter>>(),
        }
    }

    pub fn selections(&self) -> Vec<usize> {
        self.adapters
            .iter()
            .map(|p| p.as_usize())
            .collect::<Vec<usize>>()
    }

    pub fn contains(&self, adapter: Adapter) -> bool {
        self.adapters.iter().any(|p| *p == adapter)
    }
}
