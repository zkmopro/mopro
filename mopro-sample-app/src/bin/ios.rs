use mopro::app_config::build;
use mopro::app_config::Target::Ios;

fn main() {
    // Library name is the name of the crate with all `-` replaced with `_`
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let library_name = crate_name.replace("-", "_");

    build(Ios, &library_name).unwrap();
}
