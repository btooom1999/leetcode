use std::collections::HashMap;

struct Solution {
    hashmap: HashMap<i32, Vec<i32>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut hashmap = HashMap::<_, Vec<i32>>::new();
        for (i, &num) in nums.iter().enumerate() {
            hashmap.entry(num).or_default().push(i as i32);
        }

        Self { hashmap }
    }

    fn pick(&self, target: i32) -> i32 {
        let indexes = &self.hashmap[&target];
        indexes[rand::random_range(0..indexes.len())]
    }
}

pub fn main() {}
