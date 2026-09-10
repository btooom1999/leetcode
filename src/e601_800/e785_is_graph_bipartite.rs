fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut colors = vec![-1; n];
    for vertex in 0..n {
        if colors[vertex] == -1 {
            let mut q = std::collections::VecDeque::from([(vertex, true)]);
            while let Some((vertex, color)) = q.pop_front() {
                colors[vertex] = color as i32;
                for &next_vertex in &graph[vertex] {
                    if colors[next_vertex as usize] == colors[vertex] {
                        return false;
                    }

                    if colors[next_vertex as usize] == -1 {
                        q.push_back((next_vertex as usize, !color));
                    }
                }
            }
        }
    }

    true
}

pub fn main() {
    let graph = vec![vec![1,2,3],vec![0,2],vec![0,1,3],vec![0,2]];
    println!("{}", is_bipartite(graph));
}
