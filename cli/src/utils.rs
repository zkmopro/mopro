use std::collections::{HashMap, HashSet};

use mopro_ffi::app_config::constants::{AndroidArch, Arch, IosArch};

use crate::{
    config::Config, constants::Platform, select::multi_select, style,
};

pub struct PlatformSelector {
    pub platforms: Vec<Platform>,
    pub archs: Vec<String>,
}

impl PlatformSelector {
    /// Construct platforms from command-line arguments and update config
    pub fn construct_with_config(selections: Vec<String>, config: &mut Config) -> Self {
        // Clear previous selections before update
        config.target_platforms = Some(HashSet::new());

        let mut platforms: Vec<Platform> = vec![];
        for s in selections {
            let platform = Platform::parse_from_str(&s);
            config.target_platforms.as_mut().unwrap().insert(s);
            platforms.push(platform);
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

    pub fn contains(&self, platform: Platform) -> bool {
        self.platforms.contains(&platform)
    }

    /// For the better UX - users don't need to select again if failed build happens in the next step,
    /// `select_archs` updates `ios` and `android` in the config file.
    pub fn select_archs(&mut self, config: &mut Config) -> HashMap<String, Vec<String>> {
        let mut archs: HashMap<String, Vec<String>> = HashMap::new();
        Self::print_architecture_message();
        self.platforms.iter().for_each(|&p| match p {
            Platform::Ios => {
                // defaults based on previous selections
                let all_ios_archs = IosArch::all_display_strings();
                let display_strings: Vec<String> = all_ios_archs
                    .iter()
                    .map(|(arch, desc)| format!("{arch} {desc}"))
                    .collect();
                let display_refs: Vec<&str> = display_strings.iter().map(|s| s.as_str()).collect();
                let defaults: Vec<bool> = if config.ios.is_none() {
                    vec![false; all_ios_archs.len()]
                } else {
                    all_ios_archs
                        .iter()
                        .map(|(arch, _)| config.ios.as_ref().unwrap().contains(arch))
                        .collect()
                };

                // clear previous selections before update
                config.ios = Some(HashSet::new());

                let sel = Self::select_multi_archs(p.as_str(), &display_refs, defaults);
                let sel_str = sel
                    .iter()
                    .map(|&i| {
                        let (arch, _) = &all_ios_archs[i];
                        config.ios.as_mut().unwrap().insert(arch.clone());
                        arch.clone()
                    })
                    .collect::<Vec<String>>();
                archs.insert(String::from(Platform::Ios.as_str()), sel_str.clone());
                self.archs.extend_from_slice(&sel_str);
            }
            Platform::Android => {
                // defaults based on previous selections
                let all_android_archs = AndroidArch::all_display_strings();
                let display_strings: Vec<String> = all_android_archs
                    .iter()
                    .map(|(arch, desc)| format!("{arch} {desc}"))
                    .collect();
                let display_refs: Vec<&str> = display_strings.iter().map(|s| s.as_str()).collect();
                let defaults: Vec<bool> = if config.android.is_none() {
                    vec![false; all_android_archs.len()]
                } else {
                    all_android_archs
                        .iter()
                        .map(|(arch, _)| config.android.as_ref().unwrap().contains(arch))
                        .collect()
                };

                // clear previous selections before update
                config.android = Some(HashSet::new());

                let sel = Self::select_multi_archs(p.as_str(), &display_refs, defaults);
                let sel_str = sel
                    .iter()
                    .map(|&i| {
                        let (arch, _) = &all_android_archs[i];
                        config.android.as_mut().unwrap().insert(arch.clone());
                        arch.clone()
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
            format!("Select {platform} architecture(s) to compile").as_str(),
            format!(
                "No architectures selected for {platform}. Please select at least one architecture.",
            )
            .as_str(),
            archs.to_vec(),
            defaults,
        )
    }

    fn print_architecture_message() {
        println!(
            "📚 To learn more about the architecture selection, \n    visit: {}",
            style::blue_bold("https://zkmopro.org/docs/architectures".to_string())
        );
    }

    pub fn contains_archs(&self, arch_strs: &[&str]) -> bool {
        arch_strs
            .iter()
            .any(|&arch| self.archs.contains(&arch.to_string()))
    }

    /// Construct architectures from command-line arguments
    pub fn construct_archs(
        &mut self,
        archs: &[String],
        config: &mut Config,
    ) -> HashMap<String, Vec<String>> {
        let mut selected_archs: HashMap<String, Vec<String>> = HashMap::new();

        // Clear previous selections before update
        config.ios = Some(HashSet::new());
        config.android = Some(HashSet::new());

        // Group architectures by platform
        let mut ios_archs = Vec::new();
        let mut android_archs = Vec::new();

        for arch in archs {
            if IosArch::all_strings().contains(&arch.as_str()) {
                ios_archs.push(arch.clone());
                config.ios.as_mut().unwrap().insert(arch.clone());
            } else if AndroidArch::all_strings().contains(&arch.as_str()) {
                android_archs.push(arch.clone());
                config.android.as_mut().unwrap().insert(arch.clone());
            }
        }

        // Add architectures to the result if the platform is selected
        if self.contains(Platform::Ios) && !ios_archs.is_empty() {
            selected_archs.insert(String::from(Platform::Ios.as_str()), ios_archs.clone());
            self.archs.extend_from_slice(&ios_archs);
        }

        if self.contains(Platform::Android) && !android_archs.is_empty() {
            selected_archs.insert(
                String::from(Platform::Android.as_str()),
                android_archs.clone(),
            );
            self.archs.extend_from_slice(&android_archs);
        }

        selected_archs
    }
}
