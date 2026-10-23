fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![(0, 0); n]; // (i-1 < i, i-1 > i)
    dp[0] = (1,1);
    let mut res = 1;
    for i in 0..n {
        for j in (0..i).rev() {
            if nums[j] < nums[i] {
                dp[i].0 = dp[i].0.max(dp[j].1 + 1);
            } else if nums[j] > nums[i] {
                dp[i].1 = dp[i].1.max(dp[j].0 + 1)
            }

            res = res.max(dp[i].0.max(dp[i].1));
        }
    }

    res
}

pub fn main() {
    let nums = [1,7,4,9,2,5].to_vec();
    println!("{}", wiggle_max_length(nums));
}
