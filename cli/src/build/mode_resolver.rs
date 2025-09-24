use crate::config::Config;
use crate::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use mopro_ffi::app_config::constants::Mode;

pub(super) fn resolve_mode(arg_mode: &Option<String>, config: &mut Config) -> anyhow::Result<Mode> {
    match arg_mode.as_deref() {
        Some(raw_mode) => match Mode::idx(raw_mode) {
            Some(idx) => {
                let mode = Mode::from_idx(idx);
                config.build_mode = Some(mode.as_str().to_string());
                Ok(mode)
            }
            None => {
                style::print_yellow(
                    "Invalid mode selected. Please choose a valid mode (e.g., 'release' or 'debug')."
                        .to_string(),
                );
                select_mode(config)
            }
        },
        None => select_mode(config),
    }
}

fn select_mode(config: &mut Config) -> anyhow::Result<Mode> {
    let theme = ColorfulTheme::default();
    let mut selection = Select::with_theme(&theme);
    let options = Mode::all_strings();

    if let Some(build_mode) = config.build_mode.as_ref() {
        if let Some(idx) = Mode::idx(build_mode.as_str()) {
            selection.default(idx);
        }
    }

    let idx = selection
        .with_prompt("Build mode")
        .items(options.as_ref())
        .interact()?;

    let mode = Mode::from_idx(idx);
    config.build_mode = Some(mode.as_str().to_string());

    Ok(mode)
}
