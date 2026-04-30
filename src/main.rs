mod p00001_two_sum;
mod p00007_reverse_integer;
mod p00008_string_to_integer_atoi;
mod p00009_palindrome;
mod p00011_container_with_most_water;
mod p00020_valid_parentheses;
mod p00026_remove_duplicates_from_sorted_array;
mod p00027_remove_element;
mod p00038_count_and_say;
mod p00066_plus_one;
mod p00067_add_binary;
mod p00069_sqrt;
mod p00088_merge_sorted_array;
mod p00167_two_sum_2_input_array_is_sorted;
mod p00190_reverse_bits;
mod p00191_number_of_1_bits;
mod p00338_counting_bits;
mod p02119_a_number_after_a_double_reversal;

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
        Some("7")=> {
            let x: i32 = -2147483648;
            let result:i32 = p00007_reverse_integer::reverse(x);
            println!("{}", result);
        }
        Some("8")=> {
            let s: String = "   -042".to_string();
            let result:i32 = p00008_string_to_integer_atoi::my_atoi(s);
            println!("{}", result);
        }
        Some("9")=> {
            let n = 1240421;
            let result:bool = p00009_palindrome::is_palindrome(n);
            println!("{}", result);
        }
        Some("11")=> {
            let height: Vec<i32> = vec![1,8,6,2,5,4,8,3,7];
            let result:i32 = p00011_container_with_most_water::max_area(height);
            println!("{}", result);
        }
        Some("20")=> {
            let s = String::from("(()()()((())))");
            let result:bool = p00020_valid_parentheses::is_valid(s);
            println!("{}", result);
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
        Some("38")=> {
            let n: i32 = 10;
            let result: String = p00038_count_and_say::count_and_say(n);
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
        Some("88")=> {
            let mut nums1 = vec![1,2,3,0,0,0];
            let m:i32=3;
            let mut nums2 = vec![2,4,6];
            let n:i32=3;
            p00088_merge_sorted_array::merge(&mut nums1, m, &mut nums2, n);
            println!("{:?}", nums1);
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
        Some("2119")=> {
            let num: i32 = 1800;
            let result: bool = p02119_a_number_after_a_double_reversal::is_same_after_reversals(num);
            println!("{}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
}