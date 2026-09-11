use std::collections::HashSet;

fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }
}

fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let n = n as usize;
    let restricted = restricted.into_iter().collect::<HashSet<_>>();

    let mut map = (0..n).collect::<Vec<_>>();
    for edge in edges {
        if !restricted.contains(&edge[0]) && !restricted.contains(&edge[1]) {
            union(&mut map, edge[0] as usize, edge[1] as usize);
        }
    }

    let mut res = 0;
    for i in 0..n {
        if find(&mut map, i) == 0 {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let n = 7;
    let edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]].into_iter().map(Vec::from).collect();
    let restricted = [4,2,1].to_vec();
    println!("{}", reachable_nodes(n, edges, restricted));
}
