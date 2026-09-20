fn maximum_possible_size(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut count = 1;

    for &num in nums.iter().skip(1) {
        if num >= max {
            count += 1;
            max = num;
        }
    }

    count
}

pub fn main() {
    let nums = [4,2,5,3,5].to_vec();
    println!("{}", maximum_possible_size(nums));
}
