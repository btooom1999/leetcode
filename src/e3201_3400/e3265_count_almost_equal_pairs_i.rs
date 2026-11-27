use std::{cmp::Reverse, collections::HashMap};

fn count_pairs(mut nums: Vec<i32>) -> i32 {
    nums.sort_by_key(|&v| Reverse(v));

    let mut hashmap = HashMap::<_, i32>::new();
    for &num in &nums {
        *hashmap.entry(num).or_default() += 1;
    }

    let mut res = 0;
    for num in nums {
        let val = hashmap.get_mut(&num).unwrap();
        *val -= 1;
        if *val == 0 {
            hashmap.remove(&num);
        }

        let mut clone = hashmap.clone();
        res += clone.remove(&num).unwrap_or(0);
        let mut num = num.to_string().into_bytes();
        let n = num.len();
        for i in 0..n {
            for j in i+1..n {
                (num[i], num[j]) = (num[j], num[i]);
                let correct_num = String::from_utf8(num.to_vec()).unwrap().parse::<i32>().unwrap();
                res += clone.remove(&correct_num).unwrap_or(0);
                (num[i], num[j]) = (num[j], num[i]);
            }
        }
    }

    res
}

pub fn main() {
    let nums = [3,12,30,17,21].to_vec();
    println!("{}", count_pairs(nums));
}
