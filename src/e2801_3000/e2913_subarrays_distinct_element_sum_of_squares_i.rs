use std::collections::HashSet;

fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for i in 0..n {
        let mut hashset = HashSet::new();
        for j in i..n {
            hashset.insert(nums[j]);
            res += (hashset.len() * hashset.len()) as i32;
        }
    }

    res
}

pub fn main() {
    let nums = [1,2,1].to_vec();
    println!("{}", sum_counts(nums));
}
