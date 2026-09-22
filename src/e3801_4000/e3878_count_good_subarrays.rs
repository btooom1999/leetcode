fn count_good_subarrays(nums: Vec<i32>) -> i64 {
    let mut right= vec![vec![]; 32];
    let n = nums.len();
    for i in (0..n).rev() {
        for k in 0..32 {
            if nums[i] >> k & 1 == 1 {
                right[k].push(i.wrapping_sub(1));
            }
        }
    }

    let mut res = 0;
    let mut left = [0; 32];
    let mut hashmap = std::collections::HashMap::new();
    for i in 0..n {
        let mut l = 0;
        let mut r = n-1;
        for k in 0..32 {
            if nums[i] >> k & 1 == 1 {
                right[k].pop();
                left[k] = i+1;
            } else {
                l = l.max(left[k]);
                r = r.min(*right[k].last().unwrap_or(&(n-1)));
            }
        }

        if let Some(&prev_l) = hashmap.get(&nums[i]) {
            l = l.max(prev_l+1);
        }

        hashmap.insert(nums[i], i);
        res += ((i+1-l) * (r+1-i)) as i64;
    }

    res
}

pub fn main() {
    // let nums = [1,3,1].to_vec();
    let nums = [0,1,1].to_vec();
    println!("{}", count_good_subarrays(nums));
}
