fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut dp = vec![vec![i32::MAX; k]; n+1];
    dp[n][0] = 0;

    for i in (0..n).rev() {
        dp[i][0] = dp[i+1][0] + nums[i];
    }

    for k in 1..k {
        for i in (0..n-k).rev() {
            let mut sum = 0;
            for j in i..n-k {
                sum += nums[j];
                dp[i][k] = dp[i][k].min(sum.max(dp[j+1][k-1]));
            }
        }
    }

    dp[0][k-1]
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    let k = 3;
    println!("{}", split_array(nums, k));
}
