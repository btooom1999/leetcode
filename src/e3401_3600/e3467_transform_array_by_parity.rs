fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_by_key(|v| (v % 2 != 0) as i32);
    nums.into_iter().map(|v| (v % 2 != 0) as i32).collect()
}

pub fn main() {
    let nums = [4,3,2,1].to_vec();
    println!("{:?}", transform_array(nums));
}
