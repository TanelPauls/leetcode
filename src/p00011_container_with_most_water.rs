pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::min;
    if height.len()==0 || height.len()==1 {
        return 0;
    }
    let mut answer = min(height[0], height[height.len()-1]) * (height.len() as i32 - 1);
    let mut start_index = 0;
    let mut end_index  = height.len()-1;
    while start_index != end_index {
        if min(height[start_index],height[end_index])*(end_index as i32 - start_index as i32) > answer {
                answer = min(height[start_index],height[end_index])*(end_index as i32 - start_index as i32);
        }
        if height[start_index]>height[end_index] {
            end_index-=1;
        }
        else {
            start_index+=1;
        }
    }

    answer
}