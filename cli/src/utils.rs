use std::collections::HashMap;

use crate::{
    constants::{Adapter, Platform, ADAPTERS, ANDROID_ARCHS, IOS_ARCHS, PLATFORMS},
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

pub struct PlatformSelector {
    pub platforms: Vec<Platform>,
}

impl PlatformSelector {
    pub fn construct(selections: Vec<String>) -> Self {
        let mut platforms: Vec<Platform> = vec![];
        for s in selections {
            platforms.push(s.as_str().into());
        }
        Self { platforms }
    }

    pub fn select() -> Self {
        let platforms = multi_select(
            "Select platform(s) to build for (multiple selection with space)",
            "No platforms selected. Please select at least one platform.",
            PLATFORMS.to_vec(),
        );

        Self {
            platforms: platforms
                .iter()
                .map(|&p| p.into())
                .collect::<Vec<Platform>>(),
        }
    }

    pub fn contains(&self, platform: Platform) -> bool {
        self.platforms.iter().any(|p| *p == platform)
    }

    pub fn select_archs(&self) -> HashMap<String, Vec<String>> {
        let mut archs: HashMap<String, Vec<String>> = HashMap::new();
        self.platforms.iter().for_each(|&p| match p {
            Platform::Ios => {
                let sel = Self::select_multi_archs(p.into(), &IOS_ARCHS);
                let sel_str = sel
                    .iter()
                    .map(|&i| IOS_ARCHS[i].to_string())
                    .collect::<Vec<String>>();
                archs.insert("iOS".to_string(), sel_str);
            }
            Platform::Android => {
                let sel = Self::select_multi_archs(p.into(), &ANDROID_ARCHS);
                let sel_str = sel
                    .iter()
                    .map(|&i| ANDROID_ARCHS[i].to_string())
                    .collect::<Vec<String>>();
                archs.insert("Android".to_string(), sel_str);
            }
            Platform::Web => {}
        });
        archs
    }

    fn select_multi_archs(platform: &str, archs: &[&str]) -> Vec<usize> {
        // At least one architecture must be selected
        multi_select(
            "Select {} architecture(s) to compile (default: all)",
            format!(
                "No architectures selected for {}. Please select at least one architecture.",
                platform
            )
            .as_str(),
            archs.to_vec(),
        )
    }
}
