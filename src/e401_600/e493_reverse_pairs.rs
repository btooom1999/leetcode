fn count_and_sort(nums: &mut [i32], scratch: &mut [i32]) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }

    let mid = n/2;
    let (left_nums, right_nums) = nums.split_at_mut(mid);
    let (left_scratch, right_scratch) = scratch.split_at_mut(mid);

    let mut res = count_and_sort(left_nums, left_scratch) + count_and_sort(right_nums, right_scratch);
    let mut l = 0;
    for &x in right_nums.iter() {
        while l < mid && left_nums[l] as i64 <= 2 * x as i64 {
            l += 1;
        }

        res += (mid - l) as i32;
    }

    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < n {
        if nums[i] > nums[j] {
            scratch[k] = nums[j];
            j += 1;
        } else {
            scratch[k] = nums[i];
            i += 1;
        }
        k += 1;
    }

    if i < mid {
        scratch[k..].copy_from_slice(&nums[i..mid]);
    } else {
        scratch[k..].copy_from_slice(&nums[j..n]);
    }

    nums.copy_from_slice(scratch);

    res
}

fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    count_and_sort(&mut nums, &mut vec![0; n])
}

pub fn main() {
    // let nums = [2147483647,2147483647,2147483647,2147483647,2147483647,2147483647].to_vec();
    let nums = [1,3,2,3,1].to_vec();
    println!("{}", reverse_pairs(nums));
}
