use std::collections::HashSet;

const MOD: i128 = 2i128.pow(61);
const BASE: i128 = 211; // require a base number larger than the numbers of a array

fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
    let n = nums.len();
    let mut hashset = HashSet::new();
    for i in 0..n {
        let mut count = 0;
        let mut hash = 0;
        for j in i..n {
            count += (nums[j] % p == 0) as i32;
            if count > k { break; }
            hash = hash * BASE % MOD + nums[j] as i128;
            hashset.insert(hash);
        }
    }

    hashset.len() as i32
}

pub fn main() {
    let nums = [43,62,113,4,144,40,111,54,98,155,26,86,132,110,120,79,74,182,11,103,170,43,37,6,86,183,128,91,182,141,63,196,99,50,21,25,123,132,24,36,75,56,41].to_vec();
    let k = 7;
    let p = 196;
    println!("{}", count_distinct(nums, k, p));
}
