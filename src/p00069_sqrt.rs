
pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let mut left = 1;
    let mut right = x / 2;
    let mut ans = 0;

    while left <= right {
        let mid = left + (right - left) / 2;

        if mid <= x / mid {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ans
}
/* pub fn my_sqrt(x: i32) -> i32 {
    let mut answer: i32 = 0;
    let mut beginning: i32 = 0;
    let mut end: i32 = x;
    let mut middle: i32;
    
    println!("{}", beginning);
    println!("{}", end);
    while beginning!=end {
        //println!("{}", beginning);
        //println!("{}", end);
        middle=beginning+((end-beginning)/2);
        println!("middle: {}", middle);
        println!(" ");
        if middle*middle==x {
            println!(" trippy x");
            return middle;
        }
        else if middle*middle>x {
            end = middle - 1;
            println!("{}", beginning);
            println!("{}", end);
        } else {
            answer = middle;
            beginning = middle + 1;
            println!("{}", beginning);
            println!("{}", end);
        }
        if beginning==end {
            println!(" trippy");
            println!("{}", beginning);
            println!("{}", end);
            if beginning*beginning==x {
                return beginning;
            }
        }
    };
    answer
} */