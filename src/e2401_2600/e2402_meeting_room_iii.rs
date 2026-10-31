use std::{cmp::Reverse, collections::BinaryHeap};

fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();

    let n = n as usize;
    let m = meetings.len();
    let mut rooms = BinaryHeap::with_capacity(n);
    for i in 0..n {
        rooms.push(Reverse(i));
    }

    let mut dp = vec![0; n];
    let mut covered_rooms = BinaryHeap::with_capacity(n);

    let mut i = 0;
    let mut time = 0i64;
    while i < m || !covered_rooms.is_empty() {
        while covered_rooms.peek().is_some_and(|&Reverse((a, _))| a <= time) {
            let room = covered_rooms.pop().unwrap().0.1;
            rooms.push(Reverse(room));
            dp[room] += 1;
        }

        while i < m && covered_rooms.len() < n && meetings[i][0] as i64 <= time {
            covered_rooms.push(Reverse((time + meetings[i][1] as i64 - meetings[i][0] as i64, rooms.pop().unwrap().0)));
            i += 1;
        }

        if i < m && covered_rooms.len() != n {
            time = meetings[i][0] as i64;
        } else if !covered_rooms.is_empty() {
            time = covered_rooms.peek().unwrap().0.0;
        }
    }

    let mut res = 0;
    for i in 1..n {
        if dp[i] > dp[res] {
            res = i;
        }
    }

    res as i32
}

pub fn main() {
    let n = 10;
    // let meetings = [[48,49],[22,30],[13,31],[31,46],[37,46],[32,36],[25,36],[49,50],[24,34],[6,41]].into_iter().map(Vec::from).collect();
    let meetings = (0..100_000).map(|i| vec![i+1, i+1+300_000]).collect::<Vec<_>>();
    println!("{}", most_booked(n, meetings));
}
