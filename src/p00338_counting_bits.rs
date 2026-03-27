pub fn count_bits(n: i32) -> Vec<i32> {
    let mut answer = vec![0; (n + 1) as usize];

    for i in 1..=n as usize {
        answer[i] = answer[i >> 1] + (i & 1) as i32;
    }

    answer
}

/* use crate::p00191_number_of_1_bits::hamming_weight;

pub fn count_bits(n: i32) -> Vec<i32> {
    let mut answer: Vec<i32> = Vec::new();

    for i in 0..(n+1) {
        answer.push(hamming_weight(i));
    }

    answer
} */