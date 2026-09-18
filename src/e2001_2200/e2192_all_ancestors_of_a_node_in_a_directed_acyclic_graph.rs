use std::collections::HashSet;

fn dfs(
    map: &Vec<Vec<usize>>,
    i: usize,
    visited: &mut Vec<bool>,
    top_sort: &mut Vec<usize>,
) {
    visited[i] = true;
    for &next_i in &map[i] {
        if !visited[next_i] {
            dfs(map, next_i, visited, top_sort);
        }
    }

    top_sort.push(i);
}

fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut map = vec![vec![]; n];

    for edge in edges {
        map[edge[0] as usize].push(edge[1] as usize);
    }

    let mut top_sort = vec![];
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&map, i, &mut visited, &mut top_sort);
        }
    }

    top_sort.reverse();
    let mut res = vec![HashSet::new(); n];
    for i in 0..n {
        for &next_i in &map[top_sort[i]] {
            let clone = res[top_sort[i]].clone();
            res[next_i].extend(clone);
            res[next_i].insert(top_sort[i]);
        }
    }

    res
        .into_iter()
        .map(|v| {
            let mut v = v.into_iter().map(|v| v as i32).collect::<Vec<_>>();
            v.sort();

            v
        })
        .collect()
}

pub fn main() {
    let n = 8;
    let edges = [[0,3],[0,4],[1,3],[2,4],[2,7],[3,5],[3,6],[3,7],[4,6]].into_iter().map(Vec::from).collect();
    println!("{:?}", get_ancestors(n, edges));
}
