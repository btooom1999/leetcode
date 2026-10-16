use std::collections::VecDeque;

fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut res = 0;
    let mut stack = VecDeque::<(i64, i64)>::new();
    let n = nums.len();
    let mut sum = 0;
    for i in 0..n {
        if i >= k {
            let front = stack.front_mut().unwrap();
            sum -= front.0;
            if front.1 == 1 {
                stack.pop_front();
            } else {
                front.1 -= 1;
            }
        }

        let mut count = 1;
        while stack.back().is_some_and(|v| v.0 >= nums[i] as i64) {
            let (prev_num, prev_count )= stack.pop_back().unwrap();
            sum -= prev_num * prev_count;
            count += prev_count;
        }

        sum += nums[i] as i64 * count;
        res += sum;
        stack.push_back((nums[i] as i64, count));
    }


    stack.clear();
    sum = 0;
    for i in 0..n {
        if i >= k {
            let front = stack.front_mut().unwrap();
            sum -= front.0;
            if front.1 == 1 {
                stack.pop_front();
            } else {
                front.1 -= 1;
            }
        }

        let mut count = 1;
        while stack.back().is_some_and(|v| v.0 <= nums[i] as i64) {
            let (prev_num, prev_count )= stack.pop_back().unwrap();
            sum -= prev_num * prev_count;
            count += prev_count;
        }

        sum += nums[i] as i64 * count;
        res += sum;
        stack.push_back((nums[i] as i64, count));
    }

    res
}

pub fn main() {
    let nums = [1,2,3].to_vec();
    let k = 2;
    println!("{}", min_max_subarray_sum(nums, k));
}
