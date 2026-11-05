fn count(
    m: usize,
    n: usize,
    matrix: &[Vec<i32>],
    num: i32,
) -> i32 {
    let mut count = 0;
    let mut row = (m-1) as i32;
    let mut col = 0;
    while row >= 0 && col < n {
        if matrix[row as usize][col] <= num {
            count += row + 1;
            col += 1;
        } else {
            row -= 1;
        }
    }

    count
}

fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut l = matrix[0][0];
    let mut r = matrix[m-1][n-1];

    while l <= r {
        let num = (l + r) / 2;
        if count(m, n, &matrix, num) < k {
            l = num + 1;
        } else {
            r = num - 1;
        }
    }

    l
}

pub fn main() {
    let matrix = [[-5, -4],[-5, -4]].into_iter().map(Vec::from).collect();
    let k = 2;
    println!("{}", kth_smallest(matrix, k));
}
