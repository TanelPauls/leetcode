pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write = 0;

    for read in 0..nums.len() {
        if nums[read] != val {
            nums[write] = nums[read];
            write += 1;
        }
    }

    write as i32
}

/* pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    println!("{:?}", nums);
    let mut k: usize = 0;
    let mut last_pos: usize = 0;
    let mut answer: i32 = 0;
    while k< nums.len(){
        if nums[k]!=val {
            answer += 1;
            nums[last_pos]=nums[k];
            last_pos+=1;
        } 
        k+=1;
        
    }
    println!("{:?}", nums);
    answer
} */