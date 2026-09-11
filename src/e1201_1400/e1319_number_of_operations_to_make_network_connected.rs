use std::collections::HashMap;

fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, hashmap: &mut HashMap<usize, i32>, a: usize, b: usize) {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a == root_b {
        *hashmap.entry(root_a).or_default() += 1;
    }

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }
}

fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut hashmap = HashMap::new();
    let mut map = (0..n).collect::<Vec<_>>();
    for connection in connections {
        union(&mut map, &mut hashmap, connection[0] as usize, connection[1] as usize);
    }

    let mut visited = vec![false; n];
    let mut res = -1;
    let mut count = 1;
    for (x, v) in hashmap {
        let k = find(&mut map, x);
        if visited[k] {
            count += v;
        } else {
            visited[k] = true;
            res += 1;
            count += v-1
        }
    }

    for i in 0..n {
        let x = map[i];
        let k = find(&mut map, x);
        if !visited[k] {
            res += 1;
            count -= 1;
            visited[k] = true;
        }
    }

    if count < 0 {
        return -1;
    }

    res
}

pub fn main() {
    let n = 4;
    let connections = [[0,1],[0,2],[1,2]].into_iter().map(Vec::from).collect();
    println!("{}", make_connected(n, connections));
}
