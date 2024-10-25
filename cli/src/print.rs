use crate::style::{self, print_bold, print_green_bold};

fn print_footer_message() {
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
    println!();
    print_green_bold("2. Run the following commands to build and run the project:".to_string());
    print_bold("   mopro build".to_string());
    print_footer_message();
}

pub(crate) fn print_build_success_message() {
    println!();
    print_green_bold("Run the following command to create templates.".to_string());
    print_bold("   mopro create".to_string());
    print_footer_message();
}

pub(crate) fn print_create_ios_success_message() {
    print_green_bold("Template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold("  You can now use the following command to open the app:".to_string());
    println!();
    print_bold("    open ios/MoproApp.xcodeproj".to_string());
    println!();
    print_green_bold("This will open the iOS project in Xcode.".to_string());
    print_footer_message();
}

pub(crate) fn print_create_android_success_message() {
    print_green_bold("Template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold("  You can now use the following command to open the app:".to_string());
    println!();
    print_bold(r"    open android -a Android\ Studio ".to_string());
    println!();
    print_green_bold("This will open the Android project in Android Studio.".to_string());
    print_footer_message();
}
