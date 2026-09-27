fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let n = nums.len();
    let mut stack = std::collections::VecDeque::new();
    let mut increment = 0;
    let mut res = n as i64 * (n+1) as i64 / 2;
    let mut r = n-1;
    for i in (0..n).rev() {
        let mut sum = 0;
        let mut count = 0;
        while let Some(&(prev_i, prev_sum, prev_count)) = stack.back() {
            if nums[i] >= nums[prev_i] {
                count += prev_count+1;
                increment -= nums[prev_i] as i64 * prev_count - prev_sum;
                sum += prev_sum + nums[prev_i] as i64;
                stack.pop_back();
            } else {
                break;
            }
        }

        increment += nums[i] as i64 * count - sum;
        stack.push_back((i, sum, count));
        while increment > k {
            let front = stack.front_mut().unwrap();
            let val1 =  nums[front.0] as i64 * front.2 - front.1;
            let val2= nums[front.0] as i64 * (front.2-1) - (front.1-nums[r] as i64);
            if increment - val1 + val2 <= k {
                res -= (n-r) as i64;
                break;
            } else {
                increment = increment - val1 + val2;
                front.1 -= nums[r] as i64;
                front.2 -= 1;
            }
            if front.2 < 0 {
                stack.pop_front();
            }
            r -= 1;
        }
    }

    res
}

pub fn main() {
    // let nums = [1000000000,1,1,1,1].to_vec();
    // let k = 1000000000;
    let nums = [6,3,1,3,6,7].to_vec();
    let k = 4;
    println!("{}", count_non_decreasing_subarrays(nums, k));
}
