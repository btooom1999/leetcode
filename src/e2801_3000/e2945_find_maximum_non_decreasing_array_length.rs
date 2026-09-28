fn find_maximum_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prefix = vec![0;n];
    for i in 0..n {
        prefix[i] = if i == 0 { 0 } else { prefix[i-1] } + nums[i] as i64;
    }

    let mut dp = vec![(0,0); n];
    dp[0] = (1, nums[0] as i64);
    for i in 1..n {
        if dp[i-1].0 > dp[i].0 {
            dp[i] = dp[i-1];
            dp[i].1 += nums[i] as i64;
        } else if dp[i-1].0 == dp[i].0 {
            dp[i].1 = dp[i].1.min(dp[i-1].1+nums[i] as i64);
        }

        let at = prefix.partition_point(|&v| v - prefix[i-1] < dp[i-1].1);
        if at<n && prefix[at]-prefix[i-1] >= dp[i-1].1 {
            dp[at].0 = dp[i-1].0+1;
            dp[at].1 = prefix[at]-prefix[i-1];
        } else if at>0 && prefix[at-1]-prefix[i-1] >= dp[i-1].1 {
            dp[at-1].0 = dp[i-1].0+1;
            dp[at-1].1 = prefix[at-1]-prefix[i-1];
        }
    }


    dp[n-1].0
}

pub fn main() {
    // let nums = [2,2].to_vec();
    let nums = [565,779,822,547,739,635,356,903,115,774,689].to_vec();
    println!("{}", find_maximum_length(nums));
}
