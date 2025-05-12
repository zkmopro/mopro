use std::collections::{HashMap, HashSet};

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
    pub archs: Vec<String>,
}

impl PlatformSelector {
    pub fn construct(selections: Vec<String>) -> Self {
        let mut platforms: Vec<Platform> = vec![];
        for s in selections {
            platforms.push(Platform::parse_from_str(&s));
        }
        Self {
            platforms,
            archs: vec![],
        }
    }

    /// For the better UX - users don't need to select again if failed build happens in the next step,
    /// `select` updates `platforms` in the config file.
    pub fn select(config: &mut Config) -> Self {
        let platforms = Platform::all_strings();
        // defaults based on previous selections or none
        let defaults: Vec<bool> = if config.target_platforms.is_none() {
            vec![false; platforms.len()]
        } else {
            platforms
                .iter()
                .map(|&platform| config.target_platforms.as_ref().unwrap().contains(platform))
                .collect()
        };

        let platform_sel = multi_select(
            "Select platform(s) to build for (multiple selection with space)",
            "No platforms selected. Please select at least one platform.",
            platforms,
            defaults,
        );

        config.target_platforms = Some(HashSet::new());

        Self {
            platforms: platform_sel
                .iter()
                .map(|&i| {
                    let p = Platform::from_idx(i);
                    config
                        .target_platforms
                        .get_or_insert_with(HashSet::new)
                        .insert(p.as_str().to_string());
                    p
                })
                .collect::<Vec<Platform>>(),
            archs: vec![],
        }
    }

    pub fn eq(&self, platforms: &Vec<Platform>) -> bool {
        self.platforms.eq(platforms)
    }

    pub fn contains(&self, platform: Platform) -> bool {
        self.platforms.contains(&platform)
    }

    /// For the better UX - users don't need to select again if failed build happens in the next step,
    /// `select_archs` updates `ios` and `android` in the config file.
    pub fn select_archs(&mut self, config: &mut Config) -> HashMap<String, Vec<String>> {
        let mut archs: HashMap<String, Vec<String>> = HashMap::new();
        self.platforms.iter().for_each(|&p| match p {
            Platform::Ios => {
                // defaults based on previous selections
                let all_ios_archs = IosArch::all_strings();
                let defaults: Vec<bool> = if config.ios.is_none() {
                    vec![false; all_ios_archs.len()]
                } else {
                    all_ios_archs
                        .iter()
                        .map(|&acrch| config.ios.as_ref().unwrap().contains(acrch))
                        .collect()
                };

                // clear previous selections before update
                config.ios = Some(HashSet::new());

                let sel = Self::select_multi_archs(p.as_str(), &all_ios_archs, defaults);
                let sel_str = sel
                    .iter()
                    .map(|&i| {
                        let arch = IosArch::from_idx(i).as_str().to_string();
                        if let Some(ref mut ios) = config.ios {
                            ios.insert(arch.clone());
                        }
                        arch
                    })
                    .collect::<Vec<String>>();
                archs.insert(String::from(Platform::Ios.as_str()), sel_str.clone());
                self.archs.extend_from_slice(&sel_str);
            }
            Platform::Android => {
                // defaults based on previous selections
                let all_android_archs = AndroidArch::all_strings();
                let defaults: Vec<bool> = if config.android.is_none() {
                    vec![false; all_android_archs.len()]
                } else {
                    all_android_archs
                        .iter()
                        .map(|&acrch| config.android.as_ref().unwrap().contains(acrch))
                        .collect()
                };

                // clear previous selections before update
                config.android = Some(HashSet::new());

                let sel = Self::select_multi_archs(p.as_str(), &all_android_archs, defaults);
                let sel_str = sel
                    .iter()
                    .map(|&i| {
                        let arch = AndroidArch::from_idx(i).as_str().to_string();
                        if let Some(ref mut android) = config.android {
                            android.insert(arch.clone());
                        }
                        arch
                    })
                    .collect::<Vec<String>>();
                archs.insert(String::from(Platform::Android.as_str()), sel_str.clone());
                self.archs.extend_from_slice(&sel_str);
            }
            Platform::Web => {}
        });
        archs
    }

    fn select_multi_archs(platform: &str, archs: &[&str], defaults: Vec<bool>) -> Vec<usize> {
        // At least one architecture must be selected
        multi_select(
            format!("Select {} architecture(s) to compile", platform).as_str(),
            format!(
                "No architectures selected for {}. Please select at least one architecture.",
                platform
            )
            .as_str(),
            archs.to_vec(),
            defaults,
        )
    }

    pub fn contains_archs(&self, arch_strs: &[&str]) -> bool {
        arch_strs
            .iter()
            .any(|&arch| self.archs.contains(&arch.to_string()))
    }
}
