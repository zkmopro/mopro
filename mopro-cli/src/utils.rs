use std::env;
use std::process::exit;

pub fn get_mopro_root() -> String {
    match env::var("MOPRO_ROOT") {
        Ok(root) => root,
        Err(_) => {
            eprintln!("Error: MOPRO_ROOT environment variable is not set.");
            eprintln!("Please set MOPRO_ROOT to point to the local checkout of mopro.");
            eprintln!("For example: export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro");
            eprintln!("Git repository: https://github.com/oskarth/mopro");
            exit(1);
        }
    }
}
