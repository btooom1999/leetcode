use std::collections::BTreeMap;

fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut btreemap = BTreeMap::<_, i32>::new();
    for &num in &nums {
        *btreemap.entry(num).or_default() += 1;
    }

    let data = btreemap.into_iter().collect::<Vec<_>>();
    let n = data.len();
    let mut tree = vec![0; n+1];
    for i in 1..=n {
        let mut k = i;
        while k <= n {
            tree[k] += data[i-1].1;
            k += k & !(k-1);
        }
    }

    let mut res = vec![];
    for i in 1..=nums.len() {
        let mut k = data.partition_point(|&(val, _)| val <= nums[i-1]);
        let mut prev = k-1;
        let mut count = 0;
        while prev > 0 {
            count += tree[prev];
            prev -= prev & !(prev-1);
        }

        res.push(count);

        while k <= n {
            tree[k] -= 1;
            k += k & !(k-1);
        }
    }

    res
}

pub fn main() {
    let nums = [0,2,1].to_vec();
    println!("{:?}", count_smaller(nums));
}
