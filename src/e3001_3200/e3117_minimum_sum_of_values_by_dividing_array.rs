use std::collections::HashMap;

fn dfs(nums: &[i32], and_values: &[i32], i: usize, j: usize, k: i32, dp: &mut Vec<Vec<HashMap<i32, i32>>>) -> i32 {
    let (n, m) = (nums.len(), and_values.len());
    if i>=n {
        if j>=m { return 0; }
        return 1e7 as i32;
    }

    if j>=m { return 1e7 as i32; }

    if k & nums[i] < and_values[j] {
        return 1e7 as i32;
    }

    if let Some(&min) = dp[i][j].get(&k) {
        return min;
    }

    let mut res = dfs(nums, and_values, i+1, j, k & nums[i], dp);
    if k & nums[i] == and_values[j] {
        res = res.min(dfs(nums, and_values, i+1, j+1, (1<<20)-1, dp) + nums[i]);
    }

    dp[i][j].insert(k,  res);
    res
}

fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
    let (n, m) = (nums.len(), and_values.len());
    let res = dfs(&nums, &and_values, 0, 0, (1<<20)-1, &mut vec![vec![HashMap::new(); m]; n]);
    if res >= 1e7 as i32 { -1 } else { res }
}

pub fn main() {
    let nums = [1,4,3,3, 2].to_vec();
    let and_values = [0,3,3,2].to_vec();
    println!("{}", minimum_value_sum(nums, and_values));
}
