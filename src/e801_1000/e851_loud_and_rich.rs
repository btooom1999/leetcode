fn dfs(
    i: usize,
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    top_sort: &mut Vec<usize>,
) {
    visited[i] = true;
    for &child in &map[i] {
        if !visited[child] {
            dfs(child, map, visited, top_sort);
        }
    }

    top_sort.push(i);
}

fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let mut map = vec![vec![]; n];
    for edge in richer {
        map[edge[0] as usize].push(edge[1] as usize);
    }

    let mut visited = vec![false; n];
    let mut top_sort = vec![];
    for i in 0..n {
        if !visited[i] {
            dfs(i, &map, &mut visited, &mut top_sort);
        }
    }

    top_sort.reverse();
    let mut res = (0..n).collect::<Vec<_>>();
    for i in 0..n {
        for &next_i in &map[top_sort[i]] {
            if quiet[res[next_i]] > quiet[res[top_sort[i]]] {
                res[next_i] = res[top_sort[i]];
            }
        }
    }

    res.into_iter().map(|v| v as i32).collect()
}

pub fn main() {
    let richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]].into_iter().map(Vec::from).collect();
    let quiet = [3,2,5,4,6,1,7,0].to_vec();
    println!("{:?}", loud_and_rich(richer, quiet));
}
