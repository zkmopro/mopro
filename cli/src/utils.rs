use crate::{
    constants::{Adapter, ADAPTERS},
    select::multi_select,
};

pub struct Platforms {
    platforms: Vec<Adapter>,
}

impl Platforms {
    pub fn construct(selections: Vec<usize>) -> Self {
        let mut platforms: Vec<Adapter> = vec![];
        for s in selections {
            platforms.push(ADAPTERS[s].into());
        }
        Self { platforms }
    }

    pub fn select() -> Self {
        let platforms = multi_select(
            "Pick the adapters you want to use (multiple selection with space)",
            "No adapters selected. Use space to select an adapter",
            ADAPTERS.to_vec(),
        );

        Self {
            platforms: platforms
                .iter()
                .map(|&p| p.into())
                .collect::<Vec<Adapter>>(),
        }
    }

    pub fn selections(&self) -> Vec<usize> {
        self.platforms
            .iter()
            .map(|p| p.as_usize())
            .collect::<Vec<usize>>()
    }

    pub fn contains(&self, adapter: Adapter) -> bool {
        self.platforms.iter().any(|p| *p == adapter)
    }
}
