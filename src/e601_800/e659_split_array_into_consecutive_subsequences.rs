use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn is_possible(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::<_, BinaryHeap<Reverse<i32>>>::new();
    for n in nums {
        let mut len = 1;
        if hashmap.contains_key(&(n-1)) {
            let lengths = hashmap.get_mut(&(n-1)).unwrap();
            let last = lengths.pop().unwrap().0;

            if lengths.is_empty() {
                hashmap.remove(&(n-1));
            }

            len += last;
        }

        hashmap.entry(n).or_default().push(Reverse(len));
    }

    hashmap.into_values().all(|v| v.into_iter().all(|v| v.0 > 2))
}

pub fn main() {
    let nums = [4,5,6,7,7,8,8,9,10,11].to_vec();
    println!("{}", is_possible(nums));
}
