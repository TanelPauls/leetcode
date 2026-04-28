pub fn is_palindrome(mut n: i64) -> bool {
    if n < 0 || (n % 10 == 0 && n != 0) {
        return false;
    }

    let mut half = 0;

    while n > half {
        half = half * 10 + n % 10;
        n /= 10;
    }

    n == half || n == half / 10
}