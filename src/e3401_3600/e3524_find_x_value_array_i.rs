fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let k = k as usize;
    let mut dp = vec![0; k];
    let mut result = vec![0; k];

    for num in nums {
        let mut next_dp = vec![0; k];
        next_dp[(num % k as i32) as usize] += 1;

        for i in 0..k {
            if dp[i] > 0 {
                next_dp[i * num as usize % k] += dp[i];
            }
        }

        dp = next_dp;
        for (i, &count) in dp.iter().enumerate() {
            result[i] += count as i64;
        }
    }

    result
}

pub fn main() {
    let nums = [1,1,2,1,1].to_vec();
    let k = 2;
    println!("{:?}", result_array(nums, k));
}
