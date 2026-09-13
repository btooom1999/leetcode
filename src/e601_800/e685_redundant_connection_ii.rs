use std::collections::{HashMap, VecDeque};

fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut map = vec![vec![]; n+1];
    for i in 0..n {
        map[edges[i][0] as usize].push(edges[i][1] as usize);
    }

    let mut hashmap = HashMap::<_, i32>::new();
    for i in 1..n {
        let mut visited = vec![false; n+1];
        let mut q = VecDeque::from(map[i].to_vec());
        for &j in &map[i] {
            visited[j] = true;
        }
        while let Some(j) = q.pop_front() {
            let val = hashmap.entry((i, j)).or_default();
            *val += 1;

            for &j in &map[j] {
                if !visited[j] {
                    visited[j] = true;
                    q.push_back(j);
                }
            }
        }
    }

    println!("{:?}", hashmap);

    vec![]
}

pub fn main() {
    let edges = [[1,2],[1,3],[2,3]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_redundant_directed_connection(edges));
}
