use console::Style;
use console::StyledObject;

fn print_text(text: String, style: Style) {
    println!("{}", style.apply_to(text))
}

pub fn print_bold(text: String) {
    let style = Style::new().bold();
    print_text(text, style);
}

pub fn print_read_bold(text: String) {
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
