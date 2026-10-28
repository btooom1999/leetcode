#[derive(Debug)]
struct NumArray {
    nums: Vec<i32>,
    blocks: Vec<i32>,
    block_size: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let block_size = (n as f64).sqrt().ceil() as usize;
        let mut blocks = vec![0; (n+block_size-1)/block_size];

        for i in 0..n {
            blocks[i/block_size] += nums[i];
        }

        Self { nums, blocks, block_size }
    }

    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let delta = val - self.nums[index];
        self.nums[index] = val;

        self.blocks[index/self.block_size] += delta;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let l = left / self.block_size;
        let r = right / self.block_size;
        let mut sum = 0;
        if l == r {
            for i in left..=right {
                sum += self.nums[i];
            }
        } else {
            for i in left..=(l+1)*self.block_size-1 {
                sum += self.nums[i];
            }

            for i in l+1..r {
                sum += self.blocks[i];
            }

            for i in r*self.block_size..=right {
                sum += self.nums[i];
            }
        }

        sum
    }
}

pub fn main() {
}
