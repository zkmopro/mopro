use console::style;
use console::Style;
use console::StyledObject;
use dialoguer::theme::ColorfulTheme;

fn print_text(text: String, style: Style) {
    println!("{}", style.apply_to(text))
}

pub fn print_bold(text: String) {
    let style = Style::new().bold();
    print_text(text, style);
}

pub fn print_red_bold(text: String) {
    let style = Style::new().red().bold();
    print_text(text, style);
}

pub fn print_green_bold(text: String) {
    let style: Style = Style::new().green().bold();
    print_text(text, style);
}

pub fn print_yellow(text: String) {
    let style = Style::new().yellow();
    print_text(text, style);
}

pub fn blue_bold(text: String) -> StyledObject<String> {
    let style = Style::new().blue().bold();
    style.apply_to(text)
}

pub fn create_custom_theme() -> ColorfulTheme {
    ColorfulTheme {
        unchecked_item_prefix: style("⬚".to_string()).for_stderr().black(),
        ..Default::default()
    }
}

pub fn print_gray_items(items: Vec<String>) {
    for item in items {
        println!("   - {}", style(item).dim());
    }
}

pub fn grey(text: &str) -> StyledObject<String> {
    style(text.to_string()).dim()
}
