struct Solution {
    nums: Vec<i32>,
}

impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut nums = self.nums.clone();
        for i in (1..self.nums.len()).rev() {
            let j = rand::random_range(0..=i);
            (nums[i], nums[j]) = (nums[j], nums[i]);
        }

        nums
    }
}

pub fn main() {}
