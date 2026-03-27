pub fn reverse_bits(n: i32) -> i32 {
    n.reverse_bits()
}

/* pub fn reverse_bits(mut n: i32) -> i32 {
    let mut result = 0;

    for _ in 0..32 {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }

    result
} */

/* pub fn reverse_bits(n: i32) -> i32 {
    let binary = format!("{:032b}", n);
    let reversed: String = binary.chars().rev().collect();
    
    u32::from_str_radix(&reversed, 2).unwrap() as i32
} */