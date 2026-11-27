use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct RandomizedCollection {
    hashmap: HashMap<i32, HashSet<usize>>,
    values: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self {
        Self { hashmap: HashMap::new(), values: Vec::new() }
    }

    fn insert(&mut self, val: i32) -> bool {
        let hashset = self.hashmap.entry(val).or_default();
        hashset.insert(self.values.len());
        self.values.push(val);

        hashset.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.hashmap.contains_key(&val) {
            return false;
        }

        let last = self.values.pop().unwrap();
        let hashset = self.hashmap.get_mut(&val).unwrap();
        if last != val {
            let i = *hashset.iter().next().unwrap();

            hashset.remove(&i);
            if hashset.is_empty() {
                self.hashmap.remove(&val);
            }

            let hashset = self.hashmap.get_mut(&last).unwrap();
            hashset.remove(&self.values.len());
            hashset.insert(i);

            self.values[i] = last;
        } else {
            hashset.remove(&self.values.len());
            if hashset.is_empty() {
                self.hashmap.remove(&val);
            }
        }

        true
    }

    fn get_random(&self) -> i32 {
        self.values[rand::random_range(0..self.values.len())]
    }
}

pub fn main() {
    let mut randomized_collection = RandomizedCollection::new();
    println!("{}", randomized_collection.insert(1));
    println!("{}", randomized_collection.remove(1));
    println!("{}", randomized_collection.insert(1));
}
