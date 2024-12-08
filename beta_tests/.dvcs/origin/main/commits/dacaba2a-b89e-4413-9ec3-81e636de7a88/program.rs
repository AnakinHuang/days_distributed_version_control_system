use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    println!("Hello, world!");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim();

    let gcd = greatest_common_divisor(12, 18);
    println!("GCD: {}", gcd);

    println!("Hello, {}! Welcome to Rust programming!", name);
}
