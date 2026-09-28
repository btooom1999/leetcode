fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let mut res = (target[0] as i64 - nums[0] as i64).abs();
    let mut increasing = target[0] >= nums[0];

    let n = nums.len();
    for i in 1..n {
        if target[i] >= nums[i] {
            if increasing {
                if target[i]-nums[i] > target[i-1]-nums[i-1] {
                    res += (target[i]-nums[i]) as i64 - (target[i-1]-nums[i-1]) as i64
                }
            } else {
                res += (target[i]-nums[i]) as i64;
                increasing = true;
            }
        } else if !increasing {
            if nums[i]-target[i] > nums[i-1]-target[i-1] {
                res += (nums[i]-target[i]) as i64 - (nums[i-1]-target[i-1]) as i64;
            }
        } else {
            res += (nums[i]-target[i]) as i64;
            increasing = false;
        }
    }

    res
}

pub fn main() {
    let nums = [3,5,1,2].to_vec();
    let target = [4,6,2,4].to_vec();
    println!("{}", minimum_operations(nums, target));
}
