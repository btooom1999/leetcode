fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut res = 0;
    let mut min_at = -1i64;
    let mut max_at = -1i64;
    let mut limit = -1;

    for (i, num) in nums.into_iter().enumerate() {
        if num < min_k || num > max_k {
            limit = i as i64;
            continue;
        }

        if num == min_k {
            min_at = i as i64;
        }

        if num == max_k {
            max_at = i as i64;
        }

        if min_at > limit && max_at > limit {
            res += min_at.min(max_at) - limit;
        }
    }

    res
}

pub fn main() {
    let nums = vec![2,1,2,3,1,1,3,1,2];
    let min_k = 1;
    let max_k = 2;
    println!("{}", count_subarrays(nums, min_k, max_k));
}
