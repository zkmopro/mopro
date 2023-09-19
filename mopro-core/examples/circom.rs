use mopro_core::middleware::circom::run_example;

fn main() {
    match run_example() {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {}", e),
    }
}
