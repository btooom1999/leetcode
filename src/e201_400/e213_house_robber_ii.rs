fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![(0,0,0); n];
    dp[0] = (nums[0], 0, 0);
    let mut res = nums[0];
    if n>1 {
        dp[1] = (0, 0, nums[1]);
        res = res.max(nums[1]);
    }

    for i in 2..n {
        if i >= 2 {
            let extra = if (i+1)%n == dp[i-2].1 { nums[dp[i-2].1] } else { 0 };
            if dp[i-2].0 + nums[i] - extra > dp[i].0 {
                dp[i].0 = dp[i-2].0;
                dp[i].1 = dp[i-2].1;
                dp[i].0 += nums[i] - extra;
            }

            if dp[i-2].2 + nums[i] > dp[i].2 {
                dp[i].2 = dp[i-2].2;
                dp[i].2 += nums[i];
            }
        }

        if i >= 3 {
            let extra = if (i+1)%n == dp[i-3].1 { nums[dp[i-3].1] } else { 0 };
            if dp[i-3].0 + nums[i] - extra > dp[i].0 {
                dp[i].0 = dp[i-3].0;
                dp[i].1 = dp[i-3].1;
                dp[i].0 += nums[i] - extra;
            }

            if dp[i-3].2 + nums[i] > dp[i].2 {
                dp[i].2 = dp[i-3].2;
                dp[i].2 += nums[i];
            }
        }

        res = res.max(dp[i].0);
        res = res.max(dp[i].2);
    }

    res
}

pub fn main() {
    let nums = [2,1,2,6,1,8,10,10].to_vec();
    println!("{}", rob(nums));
}
