fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) -> bool {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a == root_b {
        return true;
    }

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }

    false
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut map = (0..=n).collect::<Vec<_>>();
    let mut idx = 0;
    for (i, edge) in edges.iter().enumerate() {
        if union(&mut map, edge[0] as usize, edge[1] as usize) {
            idx = i;
        }
    }

    edges[idx].clone()
}

pub fn main() {
    let edges = [[1,2],[1,3],[2,3]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_redundant_connection(edges));
}
