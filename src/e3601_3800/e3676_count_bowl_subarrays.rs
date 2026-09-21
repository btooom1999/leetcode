fn bowl_subarrays(nums: Vec<i32>) -> i64 {
    let mut stack = vec![];
    let n = nums.len();
    let mut res = 0;
    for i in (0..n).rev() {
        while stack.last().is_some_and(|&k| nums[k] < nums[i]) {
            let k = stack.pop().unwrap();
            if k-i > 1 {
                res += 1;
            }
        }

        if let Some(&k) = stack.last() && nums[k] > nums[i] && k-i>1 {
            res += 1;
        }

        stack.push(i);
    }

    res
}

pub fn main() {
    let nums = [2,5,3,1,4].to_vec();
    println!("{}", bowl_subarrays(nums));
}
