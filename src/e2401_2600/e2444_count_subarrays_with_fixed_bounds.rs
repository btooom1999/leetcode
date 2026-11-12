fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let n = nums.len();
    let mut count_min = 0;
    let mut count_max = 0;
    let mut res = 0;
    for i in 0..n {
        if nums[i] < min_k || nums[i] > max_k {
            count_min = 0;
            count_max = 0;
        } else if nums[i] == max_k {
            res += count_min;
            count_max += 1;
        } else if nums[i] == min_k {
            res += count_max;
            count_min += 1;
        } else {
            res += count_min * count_max;
        }
    }

    res
}

pub fn main() {
    let nums = [35054,398719,945315,945315,820417,945315,35054,945315,171832,945315,35054,109750,790964,441974,552913].to_vec();
    let min_k = 35054;
    let max_k = 945315;
    println!("{}", count_subarrays(nums, min_k, max_k));
}
