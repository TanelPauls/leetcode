mod p00027_remove_element;
mod p00029_remove_duplicates_from_sorted_array;
mod p00066_plus_one;
mod p00067_add_binary;
mod p00069_sqrt;

use std::env;

fn main() {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("27")=> {
            let mut nums = vec![1,1,5,2,2,5,5];
            let val: i32 = 5;
            let result = p00027_remove_element::remove_element(&mut nums, val);
            println!("{}", result);
        }
        Some("29")=> {
            let mut nums = vec![1,1,2,2,2,5,5];
            let result = p00029_remove_duplicates_from_sorted_array::remove_duplicates(&mut nums);
            println!("{}", result);
        }
        Some("66") => {
            let result = p00066_plus_one::plus_one(vec![9, 7, 9, 9]);
            println!("{:?}", result);
        }
        Some("67") => {
            let result = p00067_add_binary::add_binary("11".to_string(), "111".to_string());
            println!("{}", result);
        }
        Some("69")=> {
            let result: i32 = p00069_sqrt::my_sqrt(2147395599);
            println!("{}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
}