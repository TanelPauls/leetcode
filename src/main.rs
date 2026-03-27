mod p00001_two_sum;
mod p00026_remove_duplicates_from_sorted_array;
mod p00027_remove_element;
mod p00066_plus_one;
mod p00067_add_binary;
mod p00069_sqrt;
mod p00167_two_sum_2_input_array_is_sorted;
mod p00190_reverse_bits;
mod p00191_number_of_1_bits;
mod p00338_counting_bits;

use std::env;

fn main() {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("1")=> {
            let nums = vec![2,7,11,15];
            let target: i32 = 9;
            let result = p00001_two_sum::two_sum(nums, target);
            println!("{:?}", result);
        }
        Some("26")=> {
            let mut nums = vec![1,1,2,2,2,5,5];
            let result = p00026_remove_duplicates_from_sorted_array::remove_duplicates(&mut nums);
            println!("{}", result);
        }
        Some("27")=> {
            let mut nums = vec![1,1,5,2,2,5,5];
            let val: i32 = 5;
            let result = p00027_remove_element::remove_element(&mut nums, val);
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
        Some("167")=> {
            let numbers = vec![2,7,11,15];
            let target: i32 =9;
            let result: Vec<i32> = p00167_two_sum_2_input_array_is_sorted::two_sum(numbers, target);
            println!("{:?}", result);
        }
        Some("190")=> {
            let result: i32 = p00190_reverse_bits::reverse_bits(43261596);
            println!("{}", result);
        }
        Some("191")=> {
            let result: i32 = p00191_number_of_1_bits::hamming_weight(2147483645);
            println!("{}", result);
        }
        Some("338")=> {
            let result: Vec<i32> = p00338_counting_bits::count_bits(5);
            println!("{:?}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
}