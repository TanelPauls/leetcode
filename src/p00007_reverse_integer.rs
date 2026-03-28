pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut result: i32 = 0;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        match result.checked_mul(10).and_then(|r| r.checked_add(digit)) {
            Some(val) => result = val,
            None => return 0,
        }
    }

    result
}


/* pub fn reverse(mut x: i32) -> i32 {
        let mut negative: bool = false;

        if x == i32::MIN {
            return 0;
        }
        if x<0{
            negative=true;
            x=-x;
        }

        let int_to_string: String = x.to_string();

        let reverse: String = int_to_string.chars().rev().collect();
        //println!("{}", int_to_string);
        //println!("{}", reverse);

        let mut string_to_int: i64 = reverse.parse::<i64>().unwrap();

        if negative {
            string_to_int=-string_to_int;
        }

        let result:i32= match i32::try_from(string_to_int) {
            Ok(value) => value,
            Err(_) => 0,
        };

        result
} */