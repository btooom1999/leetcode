fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let (n, m) = (grid.len(), grid[0].len());
    let mut counts = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            counts[j] = counts[j].max(grid[i][j].to_string().len() as i32);
        }
    }

    counts
}

pub fn main() {
    let grid = [[1],[22],[333]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_column_width(grid));
}
