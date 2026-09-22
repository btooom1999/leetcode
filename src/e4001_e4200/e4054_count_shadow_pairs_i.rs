fn shadow_pairs(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut res = 0;
    let mut stack = Vec::new();
    for i in (0..n).rev() {
        let mut count = 0;
        while let Some(&(k, c)) = stack.last() {
            if nums[k] >= nums[i] {
                if nums[k] == nums[i] {
                    count += c;
                }
                stack.pop();
            } else {
                break;
            }
        }


        let k = stack.last().map_or(n, |v| v.0);
        res += (k-1-i-count) as i64;
        stack.push((i, count+1));
    }

    res
}

pub fn main() {
    let nums = [6,7,6,6,7].to_vec();
    println!("{}", shadow_pairs(nums));
}
