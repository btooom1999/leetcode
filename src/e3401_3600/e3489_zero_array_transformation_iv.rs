fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for (i, &num) in nums.iter().enumerate() {
        if num == 0 { continue; }
        let num = num as usize;
        let mut dp = vec![false; num+1];
        dp[0] = true;
        let mut flag = false;
        for (j, query) in queries.iter().enumerate() {
            let mut new_dp = dp.clone();
            let (a, b, c) = (query[0] as usize, query[1] as usize, query[2] as usize);
            for k in c..=num {
                if a <= i && i <= b && !new_dp[k] {
                    new_dp[k] = dp[k-c];
                }
            }

            if new_dp[num] {
                max = max.max(j+1);
                flag = true;
                break;
            }

            dp = new_dp;
        }

        if !flag { return -1; }
    }

    max as i32
}

pub fn main() {
    let nums = [0].to_vec();
    let queries = [[0,0,10],[0,0,5],[0,0,2],[0,0,2],[0,0,1],[0,0,2],[0,0,2]].into_iter().map(Vec::from).collect();
    println!("{}", min_zero_array(nums, queries));
}
