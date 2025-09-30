use crate::style;
use crate::style::print_bold;
use crate::style::print_green_bold;

pub fn print_footer_message() {
    println!();
    println!(
        "📚 To learn more about mopro, visit: {}",
        style::blue_bold("https://zkmopro.org".to_string())
    );
    println!();
    println!("Happy coding! 🚀");
}

pub(crate) fn print_init_instructions(project_name: String) {
    println!(
        "🚀 Project '{}' initialized successfully! 🎉",
        &project_name
    );
    println!();
    println!("To get started, follow these steps:");
    println!();
    print_green_bold("1. Navigate to your project directory:".to_string());
    print_bold(format!("   cd {}", &project_name));
    print_footer_message();
}

pub(crate) fn print_build_success_message() {
    println!();
    print_green_bold("Run the following command to create templates.".to_string());
    print_bold("   mopro create".to_string());
    println!();
    print_green_bold(
        "If you already have a project, you can run the following command to update the bindings:"
            .to_string(),
    );
    print_bold("   mopro update".to_string());
    print_footer_message();
}

pub(crate) fn print_update_success_message() {
    println!();
    print_green_bold("Bindings updated successfully!".to_string());
    print_footer_message();
}
