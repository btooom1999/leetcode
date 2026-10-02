fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = vec![];
    let mut at = 0;
    let n = nums.len();
    for i in 0..n {
        if i+1 == n || nums[i]+1 != nums[i+1] {
            if nums[at] == nums[i] {
                res.push(nums[at].to_string());
            } else {
                res.push(format!("{}->{}", nums[at], nums[i]));
            }
            at = i+1;
        }
    }

    res
}

pub fn main() {
    let nums = [0,1,2,4,5,7].to_vec();
    println!("{:?}", summary_ranges(nums));
}
