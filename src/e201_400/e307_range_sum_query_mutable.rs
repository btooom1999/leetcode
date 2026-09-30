#[derive(Debug)]
struct NumArray {
    tree: Vec<i32>,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; n+1];
        for i in 1..=n {
            let mut idx = i;
            while idx <= n {
                tree[idx] += nums[i-1];
                idx += idx & !(idx-1);
            }
        }

        Self { tree, nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut index = (index+1) as usize;
        let delta = val - self.nums[index-1];
        self.nums[index-1] = val;

        while index < self.tree.len() {
            self.tree[index] += delta;
            index += index & !(index-1);
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut right = (right+1) as usize;

        let mut right_sum = 0;
        while right > 0 {
            right_sum += self.tree[right];
            right -= right & !(right-1);
        }

        let mut left = left as usize;
        let mut left_sum = 0;
        while left > 0 {
            left_sum += self.tree[left];
            left -= left & !(left-1);
        }

        right_sum-left_sum
    }
}

pub fn main() {
}
