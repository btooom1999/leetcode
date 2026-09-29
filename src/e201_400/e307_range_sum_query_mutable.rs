#[derive(Debug)]
struct NumArray {
    sum: Vec<i32>,
    pos: Vec<usize>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut num_array = Self { sum: vec![0; 4*n], pos: vec![0; n] };

        num_array.build(&nums, 0, n-1, 0);
        num_array
    }

    fn build(&mut self, nums: &[i32], low: usize, high: usize, pos: usize) -> i32 {
        if low == high {
            self.pos[low] = pos;
            self.sum[pos] = nums[low];
            return self.sum[pos];
        }

        let m = (low+high)/2;
        let a = Self::build(self, nums, low, m, 2*pos+1);
        let b = Self::build(self, nums, m+1, high, 2*pos+2);

        self.sum[pos] = a + b;
        self.sum[pos]
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut idx = self.pos[index as usize];
        let delta = val - self.sum[idx];

        while idx != 0 {
            self.sum[idx] += delta;
            idx = (idx-1)/2;
        }

        self.sum[idx] += delta;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query_sum_range(left as usize, right as usize, 0, self.pos.len()-1, 0)
    }

    fn query_sum_range(&self, q_left: usize, q_right: usize, left: usize, right: usize, pos: usize) -> i32 {
        if q_left <= left && right <= q_right {
            return self.sum[pos];
        }

        if q_left > right || q_right < left {
            return 0;
        }

        let m = (left+right)/2;
        self.query_sum_range(q_left, q_right, left, m, 2*pos+1) + self.query_sum_range(q_left, q_right, m+1, right, 2*pos+2)
    }
}

pub fn main() {
}
