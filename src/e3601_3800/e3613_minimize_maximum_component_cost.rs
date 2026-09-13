use std::cmp::Reverse;

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
        map[root_b] = map[root_a];
    } else {
        map[root_a] = map[root_b];
    }
}

fn check(edges: &[Vec<i32>], n: usize, k: i32) -> (bool, i32) {
    let mut map = (0..n).collect::<Vec<_>>();
    for edge in edges {
        union(&mut map, edge[0] as usize, edge[1] as usize);
    }

    let mut count = 0;
    for i in 0..n {
        count += (i == map[i]) as i32;
    }

    (count <= k, edges.get(0).map_or(0, |v| v[2]))
}

fn min_cost(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
    edges.sort_by_key(|v| Reverse(v[2]));

    let mut l = 0;
    let mut r = edges.len() as i32;
    let mut res = i32::MAX;
    while l <= r {
        let m = (l+r)/2;
        let val = check(&edges[m as usize..], n as usize, k);
        if val.0 {
            l = m+1;
            res = res.min(val.1);
        } else {
            r = m-1;
        }
    }

    if res == i32::MAX { 0 } else { res }
}

pub fn main() {
    let n = 4;
    let edges = [[0,1,5],[1,2,5],[2,3,5]].into_iter().map(Vec::from).collect();
    let k = 4;
    println!("{}", min_cost(n, edges, k));
}
