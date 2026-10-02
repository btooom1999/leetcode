fn count_and_sort(prefix: &mut [i64], scratch: &mut [i64], lower: i64, upper: i64) -> i32 {
    let n = prefix.len();
    if n < 2 {
        return 0;
    }

    let mid = n/2;
    let (left_prefix, right_prefix) = prefix.split_at_mut(mid);
    let (left_scratch, right_scratch) = scratch.split_at_mut(mid);

    let mut res = count_and_sort(left_prefix, left_scratch, lower, upper) + count_and_sort(right_prefix, right_scratch, lower, upper);
    let (mut l, mut h) = (0, 0);
    for &x in right_prefix.iter() {
        while l < mid && x - left_prefix[l] > upper {
            l += 1;
        }

        while h < mid && x - left_prefix[h] >= lower {
            h += 1;
        }

        res += (h - l) as i32;
    }

    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < n {
        if prefix[i] > prefix[j] {
            scratch[k] = prefix[j];
            j += 1;
        } else {
            scratch[k] = prefix[i];
            i += 1;
        }

        k += 1;
    }

    if i < mid {
        scratch[k..].copy_from_slice(&prefix[i..mid]);
    } else {
        scratch[k..].copy_from_slice(&prefix[j..n]);
    }

    prefix.copy_from_slice(scratch);

    res
}

fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let n = nums.len()+1;
    let mut prefix = vec![0; n];
    for i in 0..n-1 {
        prefix[i+1] = prefix[i] + nums[i] as i64;
    }

    count_and_sort(&mut prefix, &mut vec![0; n], lower as i64, upper as i64)
}

pub fn main() {
    let nums = [-2,5,-1].to_vec();
    let lower = -2;
    let upper = 2;
    println!("{}", count_range_sum(nums, lower, upper));
}
