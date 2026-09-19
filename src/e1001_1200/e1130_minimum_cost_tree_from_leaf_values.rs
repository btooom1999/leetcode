fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut dp = vec![vec![(0, 0, i32::MAX); n]; n];

    for i in 0..n {
        dp[i][i] = (arr[i], arr[i], 0);
    }

    for k in 1..n {
        for i in 0..n-k {
            let n = i+k;
            for j in 0..k {
                let left = dp[i][n-j-1];
                let right = dp[n-j][n];
                let mut cur = (left.0.max(left.1), right.0.max(right.1), left.2 + right.2);
                cur.2 += cur.0 * cur.1;

                if cur.2 < dp[i][i+k].2 {
                    dp[i][i+k] = cur;
                }
            }
        }
    }

    dp[0][n-1].2
}

pub fn main() {
    let arr = [3,7,2,12,15,10,3,9].to_vec();
    // let arr = [6,2,4].to_vec();
    println!("{}", mct_from_leaf_values(arr));
}
