const MOD: i64 = 1_000_000_007;

fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut diff = vec![0; n+1];

    for request in &requests {
        diff[request[0] as usize] += 1;
        diff[request[1] as usize + 1] -= 1;
    }

    for i in 1..n {
        diff[i] += diff[i-1];
    }

    nums.sort_unstable_by(|a, b| b.cmp(a));
    diff.sort_unstable_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..n {
        sum = (sum + nums[i] as i64 * diff[i].max(0) % MOD) % MOD;
    }

    sum as i32
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    let requests = [[1,3],[0,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_sum_range_query(nums, requests));
}
