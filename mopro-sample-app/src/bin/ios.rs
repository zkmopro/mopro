use mopro::app_config::build;
use mopro::app_config::Target::Ios;

fn main() {
    build(Ios).unwrap();
}
