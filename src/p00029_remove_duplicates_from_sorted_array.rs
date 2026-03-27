pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut write = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[write - 1] {
            nums[write] = nums[i];
            write += 1;
        }
    }

    write as i32
}

/* pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //println!("{:?}", nums);
    //println!(" ");
    let mut k: usize = 0;
    let mut answer: usize = 1;
    let mut last_unique_element: i32 = nums[0];
    while k< nums.len(){
        //println!("{}", nums[k]);
        if nums[k]!=last_unique_element {
            nums[answer]=nums[k];
            answer += 1;
            last_unique_element=nums[k];
            //println!("{:?}", nums);
        }

        k+=1;
    }
    //println!(" ");
    //println!("{:?}", nums);
    answer as i32
} */