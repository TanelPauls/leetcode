mod p00066_plus_one;

use std::env;

fn main() {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("66") => {
            let result = p00066_plus_one::plus_one(vec![9, 7, 9, 9]);
            println!("{:?}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
}