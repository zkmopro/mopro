use crate::config::Config;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use mopro_ffi::app_config::constants::Mode;

pub(super) fn select_mode(config: &mut Config) -> anyhow::Result<Mode> {
    let theme = ColorfulTheme::default();
    let mut selection = Select::with_theme(&theme);

    // Get default based on previous selection.
    if let Some(build_mode) = config.build_mode.clone() {
        if let Some(idx) = Mode::idx(build_mode.as_ref()) {
            selection.default(idx);
        }
    };

    let idx = selection
        .with_prompt("Build mode")
        .items(Mode::all_strings().as_ref())
        .interact()?;

    // update user's selection
    config.build_mode = Some(Mode::from_idx(idx).as_str().to_string());

    Ok(Mode::from_idx(idx))
}
