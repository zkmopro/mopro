use crate::style;
use crate::style::create_custom_theme;
use dialoguer::MultiSelect;

pub fn multi_select(prompt: &str, warning: &str, items: Vec<&str>) -> Vec<usize> {
    let theme = create_custom_theme();
    loop {
        let selection = MultiSelect::with_theme(&theme)
            .with_prompt(prompt)
            .items(items.as_ref())
            .interact()
            .unwrap();

        // If any option is selected, return directly.
        if !selection.is_empty() {
            return selection;
        }
        style::print_yellow(String::from(warning));
    }
}
