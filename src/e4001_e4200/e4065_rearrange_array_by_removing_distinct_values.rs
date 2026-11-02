use std::collections::BTreeMap;

fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut btreemap = BTreeMap::<_, usize>::new();
    for &num in &nums {
        *btreemap.entry(num).or_default() += 1;
    }

    let mut ans = vec![];
    while !btreemap.is_empty() {
        for (&num, count) in btreemap.iter_mut() {
            ans.push(num);
            *count -= 1;
        }

        btreemap.retain(|_, b| *b > 0);
    }

    ans
}

pub fn main() {
    let nums = [3,1,3,2,1,3].to_vec();
    println!("{:?}", rearrange_array(nums));
}
