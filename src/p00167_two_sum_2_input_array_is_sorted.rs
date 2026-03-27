pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut small_index:usize=0;
    let mut big_index:usize=numbers.len();
    let mut answer:Vec<i32>=Vec::new();

    if big_index<=1{
        return vec![];
    }
    while big_index>small_index {
        if numbers[small_index]+numbers[big_index-1]==target{
            answer.push(small_index as i32);
            answer.push(big_index as i32);
            answer[0]+=1;
            return answer; 
        } else if numbers[small_index]+numbers[big_index-1]<target{
            small_index+=1;
        } else {
            big_index-=1;
        }
    }
    
    vec![]
}