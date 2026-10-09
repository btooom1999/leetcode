use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

#[derive(Debug)]
struct LazySegmentTree {
    n: usize,
    sum: Vec<i64>,
    sum_squares: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    fn new(n: usize) -> Self {
        let mut initial_n = 1;
        while initial_n < n {
            initial_n <<= 1;
        }

        Self { n: initial_n, sum: vec![0; initial_n*2-1], sum_squares: vec![0; initial_n*2-1], lazy: vec![0; initial_n*2-1] }
    }

    fn calc(&mut self, l: usize, h: usize, pos: usize, delta: i64) {
        let len = (h-l+1) as i64;
        let result = 2 * delta % MOD * self.sum[pos] % MOD + delta * delta % MOD * len % MOD;

        self.sum_squares[pos] = (self.sum_squares[pos] + result) % MOD;

        self.sum[pos] = (self.sum[pos] + delta * len % MOD) % MOD;

        self.lazy[pos] = (self.lazy[pos] + delta) % MOD;
    }

    fn update_lazy(&mut self, l: usize, h: usize, pos: usize) {
        if self.lazy[pos] > 0 {
            let m = (l+h)/2;
            self.calc(l, m, pos*2+1, self.lazy[pos]);
            self.calc(m+1, h, pos*2+2, self.lazy[pos]);

            self.lazy[pos] = 0;
        }
    }

    fn update(&mut self, l: usize, h: usize, ql: usize, qh: usize, pos: usize, delta: i64) {
        if l > qh || h < ql {
            return;
        }

        if ql <= l && h <= qh {
            self.calc(l, h, pos, delta);
            return;
        }

        self.update_lazy(l, h, pos);

        let m = (l + h)/2;
        self.update(l, m, ql, qh, pos*2+1, delta);
        self.update(m+1, h, ql, qh, pos*2+2, delta);
        self.sum[pos] = (self.sum[pos*2+1] + self.sum[pos*2+2]) % MOD;
        self.sum_squares[pos] = (self.sum_squares[pos*2+1] + self.sum_squares[pos*2+2]) % MOD;
    }
}

fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut tree = LazySegmentTree::new(n);
    let mut hashmap = HashMap::new();
    let mut res = 0;
    for r in 0..n {
        let last = hashmap.get(&nums[r]).map_or(-1, |&v| v as i32);
        tree.update(0, tree.n-1, (last+1) as usize, r, 0, 1);
        res = ((res as i64 + tree.sum_squares[0]) % MOD) as i32;
        hashmap.insert(nums[r], r);
    }

    res
}

pub fn main() {
    let nums = [1,2,1].to_vec();
    println!("{}", sum_counts(nums));
}
