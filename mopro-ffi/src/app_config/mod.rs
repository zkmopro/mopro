use std::process::Command;

pub const UDL: &str = include_str!("../mopro.udl");

pub fn install_archs() {
    let archs = vec![
        "x86_64-apple-ios",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ];
    for arch in archs {
        // install is idempotent
        Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(arch)
            .spawn()
            .expect("Failed to spawn rustup, is it installed?")
            .wait()
            .expect(format!("Failed to install target architecture {}", arch).as_str());
    }
}
