use std::collections::HashSet;

fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums1 = nums1.into_iter().collect::<HashSet<_>>();
    let nums2 = nums2.into_iter().collect::<HashSet<_>>();
    let n = nums1.len();

    let a = nums1.difference(&nums2).count();
    let b = nums2.difference(&nums1).count();

    let c = nums1.intersection(&nums2).count();
    let a = a.min(n/2);
    let b = b.min(n/2);

    (a+b+c).min(n) as i32
}

pub fn main() {
    let nums1: Vec<i32> = vec![1,2,3,4,5,6];
    let nums2: Vec<i32> = vec![2,3,2,3,2,3];
    println!("{}", maximum_set_size(nums1, nums2));
}
