fn dfs(
    map: &Vec<Vec<usize>>,
    i: usize,
    top_sort: &mut Vec<i32>,
    visited: &mut Vec<i32>,
) -> bool {
    visited[i] = 0;
    for &next_i in &map[i] {
        if visited[next_i] == 0 {
            *top_sort = vec![];
            return false;
        }

        if visited[next_i] == -1 && !dfs(map, next_i, top_sort, visited) { return false; }
    }

    visited[i] = 1;
    top_sort.push(i as i32);
    true
}

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut map = vec![vec![]; n];

    for courses in &prerequisites {
        let (a, b) = (courses[0] as usize, courses[1] as usize);
        map[b].push(a);
    }

    let mut top_sort = vec![];
    let mut visited = vec![-1; n];
    for i in 0..n {
        if visited[i] == -1 && !dfs(&map, i, &mut top_sort, &mut visited) {
            return top_sort;
        }
    }

    top_sort.reverse();
    top_sort
}

pub fn main() {
    let num_courses = 4;
    let prerequisites = [[1,0],[2,0],[3,1],[3,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_order(num_courses, prerequisites));
}
