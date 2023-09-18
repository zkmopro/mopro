use mopro_core::middleware::circom;
use mopro_core::MoproError;

// name of the .udl file
uniffi::include_scaffolding!("mopro_uniffi");

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

fn run_example() -> Result<(), MoproError> {
    circom::run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
