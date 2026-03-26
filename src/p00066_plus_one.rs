pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    //println!("{:?}", digits);
    let length = digits.len();
    //println!("{}", length);
    for i in (0..length).rev() {
        //println!("{}", i);
        
        if digits[i]!=9 {
            digits[i]+=1;
            break
        } else {
            digits[i]=0;
        }
        //println!("{}", digits[i]);
    }
    if digits[0]==0 {
        digits.insert(0, 1);
    }
    //println!("{:?}", digits);
    return digits;
}