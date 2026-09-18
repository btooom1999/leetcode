use std::collections::VecDeque;

fn create_level(
    level: &mut Vec<i32>,
    map: &Vec<Vec<usize>>,
    i: usize,
    visited: &mut Vec<bool>,
) -> i32 {
    let mut max  = 1;
    visited[i] = true;
    for &next_i in &map[i] {
        if !visited[next_i] {
            max = max.max(1+create_level(level, map, next_i, visited));
        }
    }

    level[i] = max;
    max
}

fn create_up_down_level(
    level: &mut Vec<i32>,
    map: &Vec<Vec<usize>>,
    i: usize,
    visited: &mut Vec<bool>,
    lv: i32,
) {
    visited[i] = true;
    level[i] = lv;
    for &next_i in &map[i] {
        if !visited[next_i] {
            create_up_down_level(level, map, next_i, visited, lv+1);
        }
    }
}

fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut map = vec![vec![]; n];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        map[a].push(b);
        map[b].push(a);
    }

    let mut level = vec![0; n];
    let mut visited = vec![false; n];
    create_level(&mut level, &map, 0, &mut visited);

    visited = vec![false; n];
    visited[0] = true;
    let mut res = (vec![], level[0]);
    let mut q = VecDeque::from([(0, 1)]);
    let mut cur_lv = 1;
    let mut parents = vec![];
    while let Some((i, lv)) = q.pop_front() {
        if lv != cur_lv {
            while let Some(i) = parents.pop() {
                level[i] = cur_lv.min(level[i]);
            }

            cur_lv = lv;
        }

        parents.push(i);

        let max_lv = lv.max(level[i]);
        println!("{} {}", i, max_lv);
        if res.1 > max_lv {
            res = (vec![i as i32], max_lv);
        } else if res.1 == max_lv {
            res.0.push(i as i32);
        }

        for &next_i in &map[i] {
            let mut max_lv = lv;
            for &j in &map[i] {
                if j != next_i {
                    max_lv = max_lv.max(1+level[j]);
                }
            }

            if !visited[next_i] {
                visited[next_i] = true;
                q.push_back((next_i, max_lv+1));
            }
        }
    }

    res.0
}

pub fn main() {
    let n = 6;
    let edges = [[3,0],[3,1],[3,2],[3,4],[5,4]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_min_height_trees(n, edges));
}
