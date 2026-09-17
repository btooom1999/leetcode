fn dfs(
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    i: usize,
    level: i32,
    min: &mut i32,
    result: &mut Vec<i32>,
) -> i32 {
    if visited.iter().all(|v| *v) {
        return 0;
    }

    visited[i] = true;
    let mut max = 0;
    for &next_i in &map[i] {
        if !visited[next_i] {
            max = max.max(dfs(map, visited, next_i, level+1, min, result));
        }
    }

    println!("{} {} {}", i, level, max);
    let current_min = level.max(max);
    if current_min < *min {
        *result = vec![i as i32];
        *min = current_min;
    } else if current_min == *min {
        result.push(i as i32);
    }

    max+1
}

fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut map = vec![vec![]; n];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        map[a].push(b);
        map[b].push(a);
    }

    let mut result = vec![];
    dfs(&map, &mut vec![false; n], 0, 0, &mut (n as i32), &mut result);

    result
}

pub fn main() {
    let n = 6;
    let edges = [[3,0],[3,1],[3,2],[3,4],[5,4]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_min_height_trees(n, edges));
}
