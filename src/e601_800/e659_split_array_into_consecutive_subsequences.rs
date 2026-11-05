use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn is_possible(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::<_, i32>::new();
    for num in nums {
        *hashmap.entry(num).or_default() += 1;
    }

    let mut hashset = [false; 2001];
    let mut heap = hashmap.into_iter().map(|v| (Reverse(v.0), v.1)).collect::<BinaryHeap<_>>();
    while let Some((Reverse(k), v)) = heap.pop() {
        let idx = (k+1000) as usize;
        if v > 0 {
            heap.push((Reverse(k), v-1));
        }
    }

    count > 2
}

pub fn main() {
    let nums = [1,2,3,3,4,5].to_vec();
    println!()
}
