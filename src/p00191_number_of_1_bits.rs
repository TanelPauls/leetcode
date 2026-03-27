pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

/* pub fn hamming_weight(n: i32) -> i32 {
    let binary = format!("{:b}", n);
    let mut answer = 0;
    for i in binary.chars() {
        if i=='1' {
            answer+=1;
        }
    };
    answer
} */

