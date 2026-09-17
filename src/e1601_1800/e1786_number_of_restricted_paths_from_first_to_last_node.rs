use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i32 = 1_000_000_007;

fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut map = vec![vec![]; n+1];
    for edge in edges {
        let (a, b, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
        map[a].push((b, weight));
        map[b].push((a, weight));
    }

    let mut dist = vec![i32::MAX; n+1];
    dist[n] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), n));
    while let Some((Reverse(d), u)) = heap.pop() {
        if d > dist[u] {
            continue;
        }

        for &(v, weight) in &map[u] {
            if dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
                heap.push((Reverse(dist[v]), v));
            }
        }
    }

    let mut dp = vec![0; n+1];
    dp[1] = 1;

    let mut visited = vec![false; n+1];
    let mut heap = BinaryHeap::new();
    heap.push((dist[1], 1));
    visited[1] = true;

    while let Some((distance, u)) = heap.pop() {
        for &(v, _) in &map[u] {
            if dist[v] < distance {
                dp[v] = (dp[v] + dp[u]) % MOD;
                if !visited[v] {
                    heap.push((dist[v], v));
                    visited[v] = true;
                }
            }
        }
    }

    dp[n]
}

pub fn main() {
    let n = 5;
    let edges = [[1,2,3],[1,3,3],[2,3,1],[1,4,2],[5,2,2],[3,5,1],[5,4,10]].into_iter().map(Vec::from).collect();
    println!("{}", count_restricted_paths(n, edges));
}
