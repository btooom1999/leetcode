use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap}};

fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let k = k as usize;
    let n = arrival.len();
    let mut dp = vec![0; k];

    let mut available = BTreeSet::new();
    for i in 0..k {
        available.insert(i);
    }

    let mut busy = BinaryHeap::new();

    let mut i = 0;
    let mut time = arrival[0];
    while i < n || !busy.is_empty() {
        while i < n {
            if arrival[i] < time {
                i += 1;
            } else {
                break;
            }
        }

        while busy.peek().is_some_and(|&Reverse((a, _))| a <= time) {
            let server = busy.pop().unwrap().0.1;
            dp[server] += 1;
            available.insert(server);
        }

        while i < n && arrival[i] <= time && busy.len() < k {
            let server = if let Some(&server) = available.range(i%k..).next() {
                server
            } else if let Some(&server) = available.range(..i%k).next() {
                server
            } else {
                usize::MAX
            };

            if server != usize::MAX {
                busy.push(Reverse((time+load[i], server)));
                available.remove(&server);
                i += 1;
            } else {
                break;
            }
        }

        if i < n && busy.len() != k {
            time = arrival[i];
        } else if !busy.is_empty() {
            time = busy.peek().unwrap().0.0;
        }
    }

    let mut res = vec![];
    let mut max = dp[0];
    for i in 0..k {
        if dp[i] == max {
            res.push(i as i32);
        } else if dp[i] > max {
            res = vec![i as i32];
            max = dp[i];
        }
    }

    res
}

pub fn main() {
    let k = 3;
    let arrival = [1,2,3,4,5].to_vec();
    let load = [5,2,3,3,3].to_vec();
    println!("{:?}", busiest_servers(k, arrival, load));
}
