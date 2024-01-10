use std::fs;
use std::io::Write;
use std::path::Path;

pub fn create_project_structure(project_name: &str) {
    // Create the base project directory
    let base_dir = Path::new(project_name);
    if !base_dir.exists() {
        fs::create_dir(base_dir).expect("Failed to create project directory");
    }

    // Create subdirectories
    let subdirs = ["circuits", "src", "test"];
    for subdir in subdirs.iter() {
        let dir_path = base_dir.join(subdir);
        fs::create_dir_all(&dir_path).expect("Failed to create subdirectory");
    }

    // Create files in their respective directories
    let file_contents = [
        ("circuits/hello.circom", "template content for hello.circom"),
        ("src/core.rs", "// Core module content"),
        ("test/hello-world.rs", "// Test cases for hello world"),
    ];

    for (file_path, content) in file_contents.iter() {
        let full_path = base_dir.join(file_path);
        let mut file = fs::File::create(&full_path).expect("Failed to create file");
        writeln!(file, "{}", content).expect("Failed to write to file");
    }

    // Create README.md
    let readme_path = base_dir.join("README.md");
    let mut readme = fs::File::create(readme_path).expect("Failed to create README.md");
    writeln!(readme, "# Project: {}", project_name).expect("Failed to write to README.md");
}
