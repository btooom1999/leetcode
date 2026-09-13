fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (n, m, k) = (grid.len(), grid[0].len(), k as i64);
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 0 {
                let mut q = std::collections::VecDeque::from([(i, j)]);
                let mut sum = grid[i][j] as i64;
                grid[i][j] = 0;
                while let Some((i, j)) = q.pop_front() {
                    for direct in [(1,0), (0,1), (0,-1), (-1,0)] {
                        let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                        let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                        if i < n && j < m && grid[i][j] > 0 {
                            sum += grid[i][j] as i64;
                            grid[i][j] = 0;
                            q.push_back((i, j));
                        }
                    }
                }

                res += (sum % k == 0) as i32;
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[0,2,1,0,0],[0,5,0,0,5],[0,0,1,0,0],[0,1,4,7,0],[0,2,0,0,8]].into_iter().map(Vec::from).collect();
    let k = 5;
    println!("{}", count_islands(grid, k));
}
