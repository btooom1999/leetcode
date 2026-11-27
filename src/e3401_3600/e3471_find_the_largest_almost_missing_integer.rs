use std::collections::HashMap;

fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<_, i32>::new();
    for &num in &nums {
        *hashmap.entry(num).or_default() += 1;
    }

    let n = nums.len();
    let mut res = -1;
    if nums.len() == k as usize {
        res = hashmap.into_keys().max().unwrap();
    } else if k > 1 {
        let a = *hashmap.get(&nums[0]).unwrap();
        let b = *hashmap.get(&nums[n-1]).unwrap();

        if a == 1 {
            res = nums[0];
        }
        if b == 1 {
            res = res.max(nums[n-1]);
        }
    } else {
        for (k, v) in hashmap {
            if v == 1 { res = res.max(k); }
        }
    }

    res
}

pub fn main() {
    let nums = [0,0,1].to_vec();
    let k = 3;
    println!("{}", largest_integer(nums, k));
}
