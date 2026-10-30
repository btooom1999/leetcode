use std::{cmp::Reverse, collections::BinaryHeap};

fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    events.sort_unstable();

    let mut i = 0;
    let mut res = 0;
    let n = events.len();
    let mut min_heap = BinaryHeap::new();
    let mut cur = 0;
    while i < n || !min_heap.is_empty() {
        if i<n && min_heap.is_empty() {
            cur = events[i][0];
        }

        while i<n && events[i][0] <= cur {
            min_heap.push(Reverse(events[i][1]));
            i += 1;
        }

        while min_heap.peek().is_some_and(|&Reverse(v)| v < cur) {
            min_heap.pop();
        }

        if min_heap.pop().is_some() {
            cur += 1;
            res += 1;
        }
    }

    res
}

pub fn main() {
    let events= [[1,2],[2,3],[3,4],[1,2]].into_iter().map(Vec::from).collect();
    println!("{}", max_events(events))
}
