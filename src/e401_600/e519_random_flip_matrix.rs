use std::collections::HashMap;

#[derive(Debug)]
struct Solution {
    total: i32,
    m: i32,
    n: i32,
    hashmap: HashMap<i32, i32>,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self { total: m*n, m, n, hashmap: HashMap::new() }
    }

    fn flip(&mut self) -> Vec<i32> {
        let r = rand::random_range(0..self.total);
        self.total -= 1;

        let actual = *self.hashmap.get(&r).unwrap_or(&r);
        let tail = *self.hashmap.get(&self.total).unwrap_or(&self.total);
        self.hashmap.insert(r, tail);

        vec![actual / self.n, actual % self.n]
    }

    fn reset(&mut self) {
        self.total = self.m * self.n;
        self.hashmap.clear();
    }
}

pub fn main() {
    let mut solution = Solution::new(3, 1);
    println!("{:?}", solution.flip());
    println!("{:?}", solution.flip());
    println!("{:?}", solution.flip());
    solution.reset();
    println!("{:?}", solution.flip());
}
