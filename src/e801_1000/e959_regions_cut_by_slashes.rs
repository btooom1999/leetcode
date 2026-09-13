fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let grid = grid.into_iter().map(|v| v.into_bytes()).collect::<Vec<_>>();
    let n = grid.len();
    let mut square = vec![vec![]; n*2];
    for i in 0..n {
        for j in 0..n {
            let mut above = vec![0; 3];
            let mut mid = vec![0; 3];
            let mut below = vec![0; 3];
            if grid[i][j] == b'\\' {
                above[0] = 1;
                mid[1] = 1;
                below[2] = 1;
            } else if grid[i][j] == b'/' {
                above[2] = 1;
                mid[1] = 1;
                below[0] = 1;
            }

            square[i*2].extend(above);
            square[i*2+1].extend(below);
        }
    }

    let mut res = 0;
    for i in 0..n*2 {
        for j in 0..n*2 {
            if square[i][j] == 0 {
                res += 1;
                let mut q = std::collections::VecDeque::from([(i, j)]);
                square[i][j] = 1;
                while let Some((i, j)) = q.pop_front() {
                    for direct in [(0,1), (1,0), (0,-1), (-1,0)] {
                        let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                        let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };

                        if i < n*2 && j < n*2 && square[i][j] == 0 {
                            square[i][j] = 1;
                            q.push_back((i, j));
                        }
                    }
                }
            }
        }
    }

    res
}

pub fn main() {
    let grid = [" /","/ "].into_iter().map(String::from).collect();
    println!("{}", regions_by_slashes(grid));
}
