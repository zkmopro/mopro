use std::collections::HashSet;
use std::sync::Once;

use mopro_ffi::app_config::constants::{AndroidArch, Arch, IosArch};

use crate::{config::Config, constants::Platform, select::multi_select, style};

#[derive(Debug, Clone)]
pub(super) struct TargetSelection {
    selections: Vec<PlatformSelection>,
}

#[derive(Debug, Clone)]
pub(super) struct PlatformSelection {
    platform: Platform,
    architectures: PlatformArchitectures,
}

#[derive(Debug, Clone)]
pub(super) enum PlatformArchitectures {
    Ios(Vec<IosArch>),
    Android(Vec<AndroidArch>),
    Web,
}

impl TargetSelection {
    pub(super) fn select_target(
        arg_platforms: &Option<Vec<String>>,
        arg_architectures: &Option<Vec<String>>,
        config: &mut Config,
    ) -> Self {
        let (platforms_from_args, invalid_platforms) = parse_platforms(arg_platforms);

        if !invalid_platforms.is_empty() {
            style::print_yellow(format!(
                "Ignoring unknown platform(s) provided via CLI arguments: {}.",
                invalid_platforms.join(", ")
            ));
        }

        let platforms = if !platforms_from_args.is_empty() {
            platforms_from_args
        } else {
            prompt_platforms(config)
        };

        let parsed_arg_arch = parse_architectures(&platforms, arg_architectures);
        if !parsed_arg_arch.invalid.is_empty() {
            style::print_yellow(format!(
                "Ignoring unknown architecture(s) provided via CLI arguments: {}.",
                parsed_arg_arch.invalid.join(", ")
            ));
        }

        let selections = resolve_architectures(&platforms, config, parsed_arg_arch);

        let selection = Self { selections };
        selection.persist(config);
        selection
    }

    pub(super) fn contains_platform(&self, platform: Platform) -> bool {
        self.selections
            .iter()
            .any(|selection| selection.platform == platform)
    }

    pub(super) fn contains_architecture(&self, arch: &str) -> bool {
        self.selections
            .iter()
            .any(|selection| selection.architectures.contains(arch))
    }

    pub(super) fn platforms(&self) -> impl Iterator<Item = Platform> + '_ {
        self.selections.iter().map(|selection| selection.platform)
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &PlatformSelection> {
        self.selections.iter()
    }

    pub(super) fn architecture_strings_for(&self, platform: Platform) -> Option<Vec<String>> {
        self.selections
            .iter()
            .find(|selection| selection.platform == platform)
            .map(|selection| selection.architectures.to_strings())
    }

    fn persist(&self, config: &mut Config) {
        let platform_set: HashSet<String> = self
            .selections
            .iter()
            .map(|selection| selection.platform.as_str().to_string())
            .collect();
        config.target_platforms = Some(platform_set);

        config.ios = self
            .architecture_strings_for(Platform::Ios)
            .map(|archs| archs.into_iter().collect());

        config.android = self
            .architecture_strings_for(Platform::Android)
            .map(|archs| archs.into_iter().collect());
    }
}

impl PlatformSelection {
    pub(super) fn platform(&self) -> Platform {
        self.platform
    }

    pub(super) fn architecture_strings(&self) -> Vec<String> {
        self.architectures.to_strings()
    }
}

impl PlatformArchitectures {
    fn to_strings(&self) -> Vec<String> {
        match self {
            PlatformArchitectures::Ios(archs) => {
                archs.iter().map(|arch| arch.as_str().to_string()).collect()
            }
            PlatformArchitectures::Android(archs) => {
                archs.iter().map(|arch| arch.as_str().to_string()).collect()
            }
            PlatformArchitectures::Web => Vec::new(),
        }
    }

    fn contains(&self, arch: &str) -> bool {
        match self {
            PlatformArchitectures::Ios(archs) => {
                archs.iter().any(|candidate| candidate.as_str() == arch)
            }
            PlatformArchitectures::Android(archs) => {
                archs.iter().any(|candidate| candidate.as_str() == arch)
            }
            PlatformArchitectures::Web => false,
        }
    }
}

struct ArgArch {
    ios: Vec<IosArch>,
    android: Vec<AndroidArch>,
    invalid: Vec<String>,
}

fn parse_platforms(arg_platforms: &Option<Vec<String>>) -> (Vec<Platform>, Vec<String>) {
    if let Some(raw_platforms) = arg_platforms {
        let mut platforms = Vec::new();
        let mut invalid = Vec::new();

        for value in raw_platforms {
            if let Some(platform) = Platform::parse_from_str(value) {
                if !platforms.contains(&platform) {
                    platforms.push(platform);
                }
            } else {
                invalid.push(value.clone());
            }
        }

        (platforms, invalid)
    } else {
        (Vec::new(), Vec::new())
    }
}

fn resolve_architectures(
    platforms: &[Platform],
    config: &Config,
    arg_arch: ArgArch,
) -> Vec<PlatformSelection> {
    let mut selections = Vec::with_capacity(platforms.len());

    let ArgArch { ios, android, .. } = arg_arch;

    if platforms.contains(&Platform::Ios) {
        let ios_platform_arch = if !ios.is_empty() {
            PlatformArchitectures::Ios(ios)
        } else {
            PlatformArchitectures::Ios(prompt_architectures(
                Platform::Ios,
                config.ios.as_ref(),
                IosArch::all_strings(),
            ))
        };

        selections.push(PlatformSelection {
            platform: Platform::Ios,
            architectures: ios_platform_arch,
        });
    }

    if platforms.contains(&Platform::Android) {
        let android_platform_arch = if !android.is_empty() {
            PlatformArchitectures::Android(android)
        } else {
            PlatformArchitectures::Android(prompt_architectures(
                Platform::Android,
                config.android.as_ref(),
                AndroidArch::all_strings(),
            ))
        };

        selections.push(PlatformSelection {
            platform: Platform::Android,
            architectures: android_platform_arch,
        });
    }

    if platforms.contains(&Platform::Web) {
        selections.push(PlatformSelection {
            platform: Platform::Web,
            architectures: PlatformArchitectures::Web,
        });
    }

    selections
}

fn prompt_platforms(config: &Config) -> Vec<Platform> {
    let platforms = Platform::all_strings();
    let defaults: Vec<bool> = platforms
        .iter()
        .map(|platform| {
            config
                .target_platforms
                .as_ref()
                .map(|set| set.contains(*platform))
                .unwrap_or(false)
        })
        .collect();

    let selected = multi_select(
        "Select platform(s) to build for (multiple selection with space)",
        "No platforms selected. Please select at least one platform.",
        platforms.clone(),
        defaults,
    );

    selected.into_iter().map(Platform::from_idx).collect()
}

fn prompt_architectures<A: Arch>(
    platform: Platform,
    previous: Option<&HashSet<String>>,
    options: Vec<&'static str>,
) -> Vec<A> {
    print_architecture_hint();

    let defaults: Vec<bool> = options
        .iter()
        .map(|option| previous.map(|set| set.contains(*option)).unwrap_or(false))
        .collect();

    let selected = multi_select(
        &format!(
            "Select architecture(s) for {} (multiple selection with space)",
            platform.binding_name()
        ),
        "No architectures selected. Please select at least one architecture.",
        options.clone(),
        defaults,
    );

    selected
        .into_iter()
        .map(|idx| A::parse_from_str(options[idx]))
        .collect()
}

fn parse_architectures(platforms: &[Platform], arg_architectures: &Option<Vec<String>>) -> ArgArch {
    let allow_ios = platforms.contains(&Platform::Ios);
    let allow_android = platforms.contains(&Platform::Android);

    let mut invalid = Vec::new();
    let mut ios_arch = Vec::new();
    let mut android_arch = Vec::new();

    if let Some(values) = arg_architectures {
        for value in values {
            if allow_ios {
                if let Some(arch) = parse_ios_arch(value) {
                    ios_arch.push(arch);
                    continue;
                }
            }

            if allow_android {
                if let Some(arch) = parse_android_arch(value) {
                    android_arch.push(arch);
                    continue;
                }
            }

            invalid.push(value.clone());
        }
    }

    ArgArch {
        ios: ios_arch,
        android: android_arch,
        invalid,
    }
}

fn parse_ios_arch(value: &str) -> Option<IosArch> {
    IosArch::all_strings()
        .into_iter()
        .find(|candidate| candidate.eq_ignore_ascii_case(value))
        .map(IosArch::parse_from_str)
}

fn parse_android_arch(value: &str) -> Option<AndroidArch> {
    AndroidArch::all_strings()
        .into_iter()
        .find(|candidate| candidate.eq_ignore_ascii_case(value))
        .map(AndroidArch::parse_from_str)
}

fn print_architecture_hint() {
    static PRINTED: Once = Once::new();
    PRINTED.call_once(|| {
        println!(
            "📚 To learn more about the architecture selection, \n    visit: {}",
            style::blue_bold("https://zkmopro.org/docs/architectures".to_string())
        );
    });
}
