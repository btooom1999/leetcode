fn number_of_subarrays(nums: Vec<i32>) -> i64 {
    let mut stack = vec![];
    let mut res = 0;
    for &num in nums.iter().rev() {
        res += 1;
        let mut count = 1;
        while let Some(&(prev_num, prev_count)) = stack.last() {
            if num >= prev_num{
                if num == prev_num {
                    res += prev_count;
                    count += prev_count;
                }

                stack.pop();
            } else {
                break;
            }
        }

        stack.push((num, count));
    }

    res
}

pub fn main() {
    let nums = [150,150,145,150,149,146,149,146,150].to_vec();
    println!("{}", number_of_subarrays(nums));
}
