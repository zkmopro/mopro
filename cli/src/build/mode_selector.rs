use crate::config::Config;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use mopro_ffi::app_config::constants::Mode;

pub(super) fn select_mode(config: &mut Config) -> anyhow::Result<Mode> {
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
