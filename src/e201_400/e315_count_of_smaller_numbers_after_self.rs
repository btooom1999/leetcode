struct Fenwick {
    tree: Vec<i32>
}

impl Fenwick {
    fn with_capacity(n: usize) -> Self {
        Self { tree: vec![0; n] }
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        let n = self.tree.len();
        while i <= n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut i: usize) -> i32 {
        let mut count = 0;
        while i > 0 {
            count += self.tree[i];
            i -= i & i.wrapping_neg();
        }

        count
    }
}

fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let offset = 10_001;
    let n = 20_001;
    let mut fenwick = Fenwick::with_capacity(n+1);
    let mut res = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        let pos = (nums[i]+offset) as usize;
        res[i] = fenwick.prefix_sum(pos-1);
        fenwick.add(pos, 1);
    }

    res
}

pub fn main() {
    let nums = [-1,-1].to_vec();
    println!("{:?}", count_smaller(nums));
}
