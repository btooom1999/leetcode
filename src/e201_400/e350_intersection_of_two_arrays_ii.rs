use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<_, (i32, i32)>::new();
    for num in nums1 {
        hashmap.entry(num).or_default().0 += 1;
    }

    for num in nums2 {
        hashmap.entry(num).or_default().1 += 1;
    }

    let mut ans = vec![];
    for (k, v) in hashmap {
        let min = v.0.min(v.1);
        ans.extend(vec![k; min as usize]);
    }

    ans
}

pub fn main() {
    let nums1 = [4,9,5].to_vec();
    let nums2 = [9,4,9,8,4].to_vec();
    println!("{:?}", intersect(nums1, nums2));
}
