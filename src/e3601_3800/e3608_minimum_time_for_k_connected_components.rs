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

fn check(edges: &[Vec<i32>], n: usize, k: i32) -> bool {
    let mut map = (0..n).collect::<Vec<_>>();
    for edge in edges {
        union(&mut map, edge[0] as usize, edge[1] as usize);
    }

    let mut count = 0;
    for i in 0..n {
        count += (i == map[i]) as i32;
    }

    count >= k

}

fn min_time(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
    edges.sort_by_key(|v| v[2]);

    let mut l = 0;
    let mut r = edges.len() as i32;

    while l <= r {
        let m = (l+r)/2;
        if check(&edges[m as usize..], n as usize, k) {
            r = m-1;
        } else {
            l = m+1;
        }
    }

    if r == -1 {
        0
    } else {
        edges[r as usize][2]
    }
}

pub fn main() {
    let n = 3;
    let edges = [[1,2,1],[0,2,2],[0,1,3]].into_iter().map(Vec::from).collect();
    let k = 3;
    println!("{}", min_time(n, edges, k));
}
