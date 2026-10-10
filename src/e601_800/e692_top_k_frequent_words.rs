use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn top_k_frequent(words: Vec<String>, mut k: i32) -> Vec<String> {
    let mut hashmap = HashMap::<_, i32>::new();
    for w in words {
        *hashmap.entry(w).or_default() += 1;
    }

    let mut heap = BinaryHeap::new();
    for (k, v) in hashmap {
        heap.push((v, Reverse(k)));
    }

    let mut res = vec![];
    while k > 0 {
        res.push(heap.pop().unwrap().1.0);
        k -= 1;
    }

    res
}

pub fn main() {
    let words = ["i","love","leetcode","i","love","coding"].into_iter().map(String::from).collect();
    let k = 2;
    println!("{:?}", top_k_frequent(words, k));
}
