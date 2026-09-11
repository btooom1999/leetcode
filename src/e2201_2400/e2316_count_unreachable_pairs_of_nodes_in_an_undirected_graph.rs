use std::collections::HashMap;

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

fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut map = (0..n).collect::<Vec<_>>();
    for edge in edges {
        union(&mut map, edge[0] as usize,edge[1] as usize);
    }

    let mut hashmap = HashMap::<_, i32>::new();
    let mut total = n as i32;
    for i in 0..n {
        let k = find(&mut map, i);
        *hashmap.entry(k).or_default() += 1;
    }

    let mut res = 0;
    for &count in hashmap.values() {
        total -= count;
        res += count as i64 * total as i64;
    }

    res
}

pub fn main() {
    let n = 3;
    let edges = [[0,1],[0,2],[1,2]].into_iter().map(Vec::from).collect();
    println!("{}", count_pairs(n, edges));
}
