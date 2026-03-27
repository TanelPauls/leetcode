pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - num;

        // Step 1: check first
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }

        // Step 2: insert after
        map.insert(num, i);
    }

    vec![]
}


/* pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    // First step create the hashmap
    for i in 0..nums.len() {
        map.insert(nums[i], i);
    }

    // this version is equivalent to first one:
    /* for (i, &num) in nums.iter().enumerate() {
        map.insert(num, i);
    } */

    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - num;

        if let Some(&j) = map.get(&complement) {
            if i != j {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
} */




/* pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        map.insert(num, i);
    }

    vec![] // Return an empty vector if no solution found
} */

/* pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut answer: Vec<i32> = Vec::new();

    for i in 0..nums.len()-1 {
        for j in i+1..nums.len(){
            if nums[i]+nums[j]==target{
                answer.push(i as i32);
                answer.push(j as i32);
                return answer
            }
        }
    };
    answer
} */