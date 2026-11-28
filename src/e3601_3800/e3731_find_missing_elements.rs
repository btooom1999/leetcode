fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();

    let mut res = vec![];
    let n = nums.len();
    for i in 1..n {
        for num in nums[i-1]+1..nums[i] {
            res.push(num);
        }
    }

    res
}

pub fn main() {
    let nums = [7,8,6,9].to_vec();
    println!("{:?}", find_missing_elements(nums));
}
