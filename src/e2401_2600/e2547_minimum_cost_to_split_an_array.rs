fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as i64;
    let mut dp = vec![i64::MAX; n+1];
    dp[n] = 0;
    for i in (0..n).rev() {
        let mut counts = [0; 1000];
        let mut sum = 0;
        for j in i..n {
            let idx = nums[j] as usize;
            counts[idx] += 1;
            if counts[idx] == 2 {
                sum += 2;
            } else if counts[idx] > 2 {
                sum += 1;
            }

            dp[i] = dp[i].min(sum+k+dp[j+1]);
        }
    }

    dp[0] as i32
}

pub fn main() {
    let nums = [1,2,1,2,1].to_vec();
    let k = 2;
    println!("{}", min_cost(nums, k));
}
