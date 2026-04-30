pub fn count_and_say(n: i32) -> String {
    if n <= 0 {
        return String::new();
    }

    let mut s = String::from("1");

    for _ in 1..n {
        let bytes = s.as_bytes();
        let mut next = String::with_capacity(s.len() * 2);

        let mut count = 1;

        for i in 1..bytes.len() {
            if bytes[i] == bytes[i - 1] {
                count += 1;
            } else {
                next.push_str(&count.to_string());
                next.push(bytes[i - 1] as char);
                count = 1;
            }
        }

        next.push_str(&count.to_string());
        next.push(bytes[bytes.len() - 1] as char);

        s = next;
    }

    s
}


/* pub fn count_and_say(n: i32) -> String {
    if n==0 {
        return "".to_string();
    }
    let mut ans_str: String = "1".to_string();
    let mut ans_vec: Vec<[u8; 2]>;
    if n==1 {
        return "1".to_string();
    }
    for _ in 1..n {
        ans_vec = counter(&ans_str);
        ans_str = collector(ans_vec);
    }
    ans_str
}

fn counter(s: &str) -> Vec<[u8; 2]> {
    let mut result = Vec::new();

    let mut chars = s.chars();

    if let Some(mut prev) = chars.next() {
        let mut count: u8 = 1;

        for c in chars {
            if c == prev {
                count += 1;
            } else {
                let digit = prev.to_digit(10).unwrap() as u8;
                result.push([count, digit]);

                prev = c;
                count = 1;
            }
        }

        let digit = prev.to_digit(10).unwrap() as u8;
        result.push([count, digit]);
    }

    result
}

fn collector(vec_in: Vec<[u8; 2]>)-> String {
    let mut ans: String = "".to_string();
    for item in vec_in {
        ans.push((b'0' + item[0]) as char);
        ans.push((b'0' + item[1]) as char);
    }
    ans
} */