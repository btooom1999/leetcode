use std::{cmp::Reverse, collections::BinaryHeap};

fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut map = vec![vec![]; n+1];

    for time in times {
        map[time[0] as usize].push((time[1] as usize, time[2]));
    }

    let mut dist = vec![i32::MAX; n+1];
    dist[0] = 0;
    dist[k] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), k));

    while let Some((Reverse(d), u)) = heap.pop() {
        if d > dist[u] {
            continue;
        }

        for &(v, weight) in &map[u] {
            if d + weight < dist[v] {
                dist[v] = d + weight;
                heap.push((Reverse(dist[v]), v));
            }
        }
    }

    if dist.contains(&i32::MAX) {
        return -1;
    }

    dist.into_iter().max().unwrap()
}

pub fn main() {
    let times = [[2,1,1],[2,3,1],[3,4,1]].into_iter().map(Vec::from).collect();
    let n = 4;
    let k = 2;
    println!("{}", network_delay_time(times, n, k));
}
