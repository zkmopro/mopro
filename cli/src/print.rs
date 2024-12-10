use crate::style;
use crate::style::print_bold;
use crate::style::print_green_bold;

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

pub(crate) fn print_create_flutter_success_message() {
    print_green_bold("Flutter template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold(
        "  You can now use the following command to open the app in Android Studio:".to_string(),
    );
    println!();
    print_bold(r"    open flutter-app -a Android\ Studio ".to_string());
    println!();
    print_green_bold("  or VS Code::".to_string());
    println!();
    print_bold(r"    code flutter-app".to_string());
    println!();
    print_green_bold(
        "To learn more about setting up the Flutter app with mopro, visit https://zkmopro.org/docs/setup/flutter-setup/".to_string(),
    );

    print_footer_message();
}

pub(crate) fn print_create_react_native_success_message() {
    print_green_bold("React Native template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold(
        "  You can now use the following command to open the app in VS Code:".to_string(),
    );
    println!();
    print_bold(r"    code react-native-app".to_string());
    println!();
    print_green_bold(
        "To learn more about setting up the React Native app with mopro, visit https://zkmopro.org/docs/setup/react-native-setup/".to_string(),
    );

    print_footer_message();
}
