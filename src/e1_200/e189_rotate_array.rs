fn rotate(nums: &mut [i32], k: i32) {
    let n = nums.len();
    for i in 0..n / 2 {
       (nums[i], nums[n-1-i]) = (nums[n-1-i], nums[i]);
    }

    let k = k as usize % n;
    for i in 0..k / 2 {
        (nums[i], nums[k-i-1]) = (nums[k-i-1], nums[i]);
    }

    for i in 0..(n - k) / 2 {
        (nums[k+i], nums[n-1-i]) = (nums[n-1-i], nums[k+i]);
    }
}

pub fn main() {
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    rotate(&mut nums, k);
    println!("{:?}", nums);
}
