use mopro_ffi::app_config::constants::{AndroidArch, Arch, IosArch};
use std::collections::HashMap;

use crate::{
    config::Config, constants::Platform, select::multi_select, style,
};

pub(super) struct TargetSelector {
    arch_map: HashMap<Platform, Vec<String>>,
}

impl TargetSelector {
    /// Create a new TargetSelector instance based on CLI arguments and existing config.
    /// If CLI arguments are not provided, it will prompt the user for selections.
    /// The selected platforms and architectures will be written back to the config
    /// for future preselection.
    pub(super) fn new_with_config(
        arg_platforms: &Option<Vec<String>>,
        arg_architectures: &Option<Vec<String>>,
        config: &mut Config,
    ) -> Self {
        let selected_platforms = if let Some(raw_platforms) = arg_platforms {
            let valid_platforms: Vec<(usize, Platform)> = raw_platforms
                .iter()
                .enumerate()
                .map(|(i, p)| (i, Platform::parse_from_str(p)))
                .filter(|(_, p)| Option::is_some(p))
                .map(|(i, p)| (i, Option::unwrap(p)))
                .collect();
            if valid_platforms.is_empty() {
                style::print_yellow(
                    "Ignoring unknown platform(s) provided via CLI arguments: {}.".to_string(),
                );
                Self::select_platform(config)
            } else {
                if valid_platforms.len() < raw_platforms.len() {
                    style::print_yellow(format!(
                        "Invalid platform(s) selected. Only {:?} platform(s) is created.",
                        &valid_platforms
                    ));
                }
                valid_platforms.iter().map(|(_, p)| *p).collect()
            }
        } else {
            Self::select_platform(config)
        };

        let mut arch_map = HashMap::new();

        if let Some(architectures) = arg_architectures {
            let mut ios_archs: Vec<String> = architectures
                .iter()
                .filter(|&arch| IosArch::all_strings().contains(&arch.as_str()))
                .cloned()
                .collect();
            let mut android_archs: Vec<String> = architectures
                .iter()
                .filter(|&arch| AndroidArch::all_strings().contains(&arch.as_str()))
                .cloned()
                .collect();

            if ios_archs.is_empty() && selected_platforms.contains(&Platform::Ios) {
                ios_archs = Self::select_architectures(Platform::Ios, config);
            }

            if android_archs.is_empty() && selected_platforms.contains(&Platform::Android) {
                android_archs = Self::select_architectures(Platform::Android, config);
            }

            arch_map.insert(Platform::Ios, ios_archs);
            arch_map.insert(Platform::Android, android_archs);

            if selected_platforms.contains(&Platform::Web) {
                arch_map.insert(Platform::Web, vec![]);
            }
        } else {
            for platform in selected_platforms.iter() {
                let archs = Self::select_architectures(*platform, config);
                arch_map.insert(*platform, archs);
            }
        }

        Self::update_config(config, selected_platforms, &mut arch_map);

        Self { arch_map }
    }

    pub(super) fn is_platform_selected(&self, platform: Platform) -> bool {
        self.arch_map.contains_key(&platform)
    }

    pub(super) fn is_architecture_selected(&self, arch: &str) -> bool {
        for arch_list in self.arch_map.values() {
            if arch_list.contains(&arch.to_string()) {
                return true;
            }
        }

        false
    }

    pub(super) fn get_arch_for_platform(&self, platform: Platform) -> Vec<&String> {
        self.arch_map
            .get(&platform)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub(super) fn get_platforms(&self) -> Vec<&Platform> {
        self.arch_map.keys().collect()
    }

    /// For the better UX - users don't need to select again if failed build happens in the next step,
    /// `select` updates `platforms` in the config file.
    fn select_platform(config: &Config) -> Vec<Platform> {
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

        platform_sel
            .iter()
            .map(|&i| Platform::from_idx(i))
            .collect::<Vec<Platform>>()
    }

    fn select_architectures(platform: Platform, config: &Config) -> Vec<String> {
        let options: Vec<&str> = match platform {
            Platform::Ios => IosArch::all_strings(),
            Platform::Android => AndroidArch::all_strings(),
            Platform::Web => return vec![],
        };

        let defaults: Vec<bool> = match platform {
            Platform::Ios => {
                if let Some(ios_arch) = config.ios.as_ref() {
                    options
                        .iter()
                        .map(|&arch| ios_arch.contains(arch))
                        .collect()
                } else {
                    vec![false; options.len()]
                }
            }
            Platform::Android => {
                if let Some(android_arch) = config.android.as_ref() {
                    options
                        .iter()
                        .map(|&arch| android_arch.contains(arch))
                        .collect()
                } else {
                    vec![false; options.len()]
                }
            }
            Platform::Web => vec![],
        };

        Self::print_architecture_message();

        let arch_sel = multi_select(
            &format!(
                "Select architecture(s) for {} (multiple selection with space)",
                platform.binding_name()
            ),
            "No architectures selected. Please select at least one architecture.",
            options.clone(),
            defaults,
        );

        arch_sel
            .iter()
            .map(|&i| options[i].to_string())
            .collect::<Vec<String>>()
    }

    fn update_config(
        config: &mut Config,
        selected_platforms: Vec<Platform>,
        arch_map: &mut HashMap<Platform, Vec<String>>,
    ) {
        config.target_platforms = Some(
            selected_platforms
                .iter()
                .map(|p| p.as_str().to_string())
                .collect(),
        );

        config.ios = arch_map
            .get(&Platform::Ios)
            .map(|v| v.iter().cloned().collect());
        config.android = arch_map
            .get(&Platform::Android)
            .map(|v| v.iter().cloned().collect());
    }

    fn print_architecture_message() {
        println!(
            "📚 To learn more about the architecture selection, \n    visit: {}",
            style::blue_bold("https://zkmopro.org/docs/architectures".to_string())
        );
    }
}
