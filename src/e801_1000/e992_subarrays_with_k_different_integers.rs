use std::collections::HashMap;

fn at_most_k(nums: &[i32], k: usize) -> i32 {
    let mut hashmap = HashMap::<_, i32>::new();
    let n = nums.len();
    let mut res = 0;
    let mut i = 0;
    for j in 0..n {
        *hashmap.entry(nums[j]).or_default() += 1;

        while hashmap.len() > k {
            let val = hashmap.get_mut(&nums[i]).unwrap();
            *val -= 1;
            if *val == 0 {
                hashmap.remove(&nums[i]);
            }
            i += 1;
        }

        res += j-i+1;
    }

    res as i32
}

fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;

    at_most_k(&nums, k) - at_most_k(&nums, k-1)
}

pub fn main() {
    let nums = [1,2,1,3,4].to_vec();
    let k = 3;
    println!("{}", subarrays_with_k_distinct(nums, k));
}
