use std::collections::HashMap;

fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let n = stones.len();
    let mut map_x = HashMap::<_, Vec<usize>>::new();
    let mut map_y = HashMap::<_, Vec<usize>>::new();

    for (i, stone) in stones.iter().enumerate() {
        map_x.entry(stone[0] as usize).or_default().push(i);
        map_y.entry(stone[1] as usize).or_default().push(i);
    }

    let mut res = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            let mut count = 0;
            visited[i] = true;
            let mut q = std::collections::VecDeque::from([i]);
            while let Some(i) = q.pop_front() {
                count += 1;
                for &x in map_x.get(&(stones[i][0] as usize)).unwrap_or(&vec![]) {
                    if !visited[x] {
                        visited[x] = true;
                        q.push_back(x);
                    }
                }

                for &y in map_y.get(&(stones[i][1] as usize)).unwrap_or(&vec![]) {
                    if !visited[y] {
                        visited[y] = true;
                        q.push_back(y);
                    }
                }
            }

            res += count - 1;
        }
    }

    res
}

pub fn main() {
    let stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]].into_iter().map(Vec::from).collect();
    println!("{}", remove_stones(stones));
}
