use std::collections::HashMap;

fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut hashmap = HashMap::<_, i32>::new();
    let n = nums.len();
    let mut res = 0;
    let mut i = -1;
    for j in 0..n {
        *hashmap.entry(nums[j]).or_default() = j as i32;

        if hashmap.len() > k {
            let (&num, &at) = hashmap.iter().min_by_key(|a| a.1).unwrap();
            i = at;
            hashmap.remove(&num);
        }

        if hashmap.len() == k {
            let min = *hashmap.values().min().unwrap();
            res += min - i;
        }
    }

    res
}

pub fn main() {
    let nums = [1,2,1,2,3].to_vec();
    let k = 2;
    println!("{}", subarrays_with_k_distinct(nums, k));
}
