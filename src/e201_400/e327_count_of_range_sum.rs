use std::collections::BTreeMap;

fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let lower = lower as i64;
    let upper = upper as i64;
    let mut btreemap = BTreeMap::<_, i32>::new();
    let mut sum = 0;
    for &num in &nums {
        sum += num as i64;
        *btreemap.entry(sum).or_default() += 1;
    }

    let data = btreemap.into_iter().collect::<Vec<_>>();
    let n = data.len();
    let mut tree = vec![0; n+1];
    for i in 0..n {
        let mut k = i+1;
        while k <= n {
            tree[k] += data[i].1;
            k += k & !(k-1);
        }
    }

    let mut res = 0;
    for i in (0..nums.len()).rev() {
        let mut k = data.partition_point(|v| v.0 <= sum);
        while k <= n {
            tree[k] -= 1;
            k += k & !(k-1);
        }

        let mut left_idx = data.partition_point(|v| sum - v.0 > upper);
        let mut right_idx= data.partition_point(|v| sum - v.0 >= upper || sum - v.0 >= lower);

        let mut left_count = 0;
        while left_idx > 0 {
            left_count += tree[left_idx];
            left_idx -= left_idx & !(left_idx-1);
        }

        let mut right_count = 0;
        while right_idx > 0 {
            right_count += tree[right_idx];
            right_idx -= right_idx & !(right_idx-1);
        }

        res += right_count-left_count + (sum >= lower && sum <= upper) as i32;
        sum -= nums[i] as i64;
    }

    res
}

pub fn main() {
    let nums = [0,0].to_vec();
    let lower = 0;
    let upper = 0;
    println!("{}", count_range_sum(nums, lower, upper));
}
