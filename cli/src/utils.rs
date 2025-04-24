use std::collections::HashMap;

use crate::{
    config::Config,
    constants::{AndroidArch, IosArch, Platform},
    init::adapter::Adapter,
    select::multi_select,
};

pub fn contains_adapter(path: &str, adapter: Adapter) -> bool {
    path.to_lowercase().contains(adapter.as_str())
}

pub struct PlatformSelector {
    pub platforms: Vec<Platform>,
}

impl PlatformSelector {
    pub fn construct(selections: Vec<String>) -> Self {
        let mut platforms: Vec<Platform> = vec![];
        for s in selections {
            platforms.push(Platform::parse_from_str(&s));
        }
        Self { platforms }
    }

    pub fn select(config: &Config) -> Self {
        let platforms = Platform::all_strings();
        // defaults based on previous selections.
        let defaults: Vec<bool> = platforms
            .iter()
            .map(|&platform| config.target_platforms.contains(platform))
            .collect();

        let platform_sel = multi_select(
            "Select platform(s) to build for (multiple selection with space)",
            "No platforms selected. Please select at least one platform.",
            platforms,
            defaults,
        );

        Self {
            platforms: platform_sel
                .iter()
                .map(|&i| Platform::from_idx(i))
                .collect::<Vec<Platform>>(),
        }
    }

    pub fn eq(&self, platforms: &Vec<Platform>) -> bool {
        self.platforms.eq(platforms)
    }

    pub fn contains(&self, platform: Platform) -> bool {
        self.platforms.contains(&platform)
    }

    pub fn select_archs(&self) -> HashMap<String, Vec<String>> {
        let mut archs: HashMap<String, Vec<String>> = HashMap::new();
        self.platforms.iter().for_each(|&p| match p {
            Platform::Ios => {
                let sel = Self::select_multi_archs(p.as_str(), &IosArch::all_strings());
                let sel_str = sel
                    .iter()
                    .map(|&i| IosArch::from_idx(i).as_str().to_string())
                    .collect::<Vec<String>>();
                archs.insert(String::from(Platform::Ios.as_str()), sel_str);
            }
            Platform::Android => {
                let sel = Self::select_multi_archs(p.as_str(), &AndroidArch::all_strings());
                let sel_str = sel
                    .iter()
                    .map(|&i| AndroidArch::from_idx(i).as_str().to_string())
                    .collect::<Vec<String>>();
                archs.insert(String::from(Platform::Android.as_str()), sel_str);
            }
            Platform::Web => {}
        });
        archs
    }

    fn select_multi_archs(platform: &str, archs: &[&str]) -> Vec<usize> {
        // At least one architecture must be selected
        multi_select(
            format!(
                "Select {} architecture(s) to compile (default: all)",
                platform
            )
            .as_str(),
            format!(
                "No architectures selected for {}. Please select at least one architecture.",
                platform
            )
            .as_str(),
            archs.to_vec(),
            vec![true; archs.len()],
        )
    }
}
