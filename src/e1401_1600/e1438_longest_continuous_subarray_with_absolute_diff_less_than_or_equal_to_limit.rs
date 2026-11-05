use std::collections::BTreeMap;

fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;

    let n = nums.len();
    let mut i = 0;
    let mut btreemap = BTreeMap::<_, i32>::new();
    for j in 0..n {
        *btreemap.entry(nums[j]).or_default() += 1;

        while let (Some((&first, _)), Some((&last, _))) = (btreemap.first_key_value(), btreemap.last_key_value()) && last-first>limit {
            ans = ans.max(j-i);
            let val = btreemap.get_mut(&nums[i]).unwrap();
            *val -= 1;
            if *val == 0 {
                btreemap.remove(&nums[i]);
            }

            i += 1;
        }
    }

    ans.max(n-i) as i32
}

pub fn main() {
    let nums = [8,2,4,7].to_vec();
    let limit = 4;
    println!("{}", longest_subarray(nums, limit));
}
