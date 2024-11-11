use std::env;

pub fn liblaika_hello() {
    println!("Hello from {}", env!("CARGO_PKG_NAME"));
    println!("{}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}
