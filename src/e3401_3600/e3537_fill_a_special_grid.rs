fn divide_and_conquer(
    grid: &mut Vec<Vec<i32>>,
    i: usize,
    j: usize,
    len: usize,
    num: &mut i32,
) {
    if len == 1 {
        grid[i][j] = *num;
        *num += 1;
        return;
    }

    let next_len = len/2;
    divide_and_conquer(grid, i, j+next_len, next_len, num);
    divide_and_conquer(grid, i+next_len, j+next_len, next_len, num);
    divide_and_conquer(grid, i+next_len, j, next_len, num);
    divide_and_conquer(grid, i, j, next_len, num);
}

fn special_grid(n: i32) -> Vec<Vec<i32>> {
    let n = 2usize.pow(n as u32);
    let mut grid = vec![vec![0; n]; n];
    divide_and_conquer(&mut grid, 0, 0, n, &mut 0);

    grid
}

pub fn main() {
    let n = 3;
    println!("{:?}", special_grid(n));
}
