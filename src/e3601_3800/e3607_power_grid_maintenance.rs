use std::collections::BTreeSet;

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

fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let c = c as usize;
    let mut dsu = (0..=c).collect::<Vec<_>>();

    for connect in connections {
        union(&mut dsu, connect[0] as usize, connect[1] as usize);
    }

    let mut map = vec![BTreeSet::new(); c+1];
    for i in 1..=c {
        let x = find(&mut dsu, i);
        map[x].insert(i);
    }

    let mut res = Vec::new();
    for query in queries {
        let (u, v) = (find(&mut dsu, query[1] as usize), query[1] as usize);
        if query[0] == 2 {
            map[u].remove(&v);
        } else if !map[u].contains(&v) {
            res.push(map[u].iter().next().map_or(-1, |v| *v as i32));
        } else {
            res.push(v as i32);
        }
    }

    res
}

pub fn main() {
    let c = 5;
    let connections = [[1,2],[2,3],[3,4],[4,5]].into_iter().map(Vec::from).collect();
    let queries = [[1,3],[2,1],[1,1],[2,2],[1,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", process_queries(c, connections, queries));
}
