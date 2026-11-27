use std::collections::HashMap;

struct RandomizedSet {
    hashmap: HashMap<i32, usize>,
    values: Vec<i32>,
    key: usize,
}

impl RandomizedSet {
    fn new() -> Self {
        Self { hashmap: HashMap::new(), values: Vec::new(), key: 0 }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.hashmap.contains_key(&val) {
            self.values.push(val);
            self.hashmap.insert(val, self.key);
            self.key += 1;
            return true;
        }

        false
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.hashmap.remove(&val) {
            let num = self.values.pop().unwrap();
            if num != val {
                self.hashmap.insert(num, i);
                self.values[i] = num;
            }
            self.key -= 1;
            return true;
        }

        false
    }

    fn get_random(&mut self) -> i32 {
        let key = rand::random_range(0..self.key);
        self.values[key]
    }
}

pub fn main() {
    let mut randomized_set = RandomizedSet::new();
    println!("{}", randomized_set.insert(0));
    println!("{}", randomized_set.remove(0));
    println!("{}", randomized_set.insert(0));
}
