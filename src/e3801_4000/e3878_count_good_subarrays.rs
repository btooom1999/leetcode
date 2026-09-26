fn count_good_subarrays(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut right = vec![n-1; n];
    let mut stack = vec![];

    for i in 0..n {
        while let Some(&j) = stack.last() {
            if nums[j] | nums[i] != nums[j] {
                right[j] = i-1;
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(i);
    }

    stack.clear();
    let mut left = vec![0; n];
    for i in (0..n).rev() {
        while let Some(&j) = stack.last() {
            if nums[j] | nums[i] != nums[j] || nums[j] == nums[i] {
                left[j] = i+1;
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(i);
    }

    let mut res = 0;
    for i in 0..n {
        res += (i - left[i] + 1) as i64 * (right[i] - i + 1) as i64;
    }

    res
}

pub fn main() {
    let nums = [1,3,1].to_vec();
    // let nums = [0,1,1].to_vec();
    println!("{}", count_good_subarrays(nums));
}
