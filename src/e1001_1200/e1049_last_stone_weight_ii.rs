pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum::<i32>();
    let n = stones.len();
    let mut dp = vec![vec![0; (sum/2+1) as usize]; n+1];
    for i in 1..=n {
        for j in 1..=(sum/2) as usize {
            if stones[i-1] as usize <= j {
                dp[i][j] = dp[i-1][j].max(stones[i-1] + dp[i-1][j-stones[i-1] as usize]);
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }

    let a = dp[n][(sum/2) as usize];
    (a-(sum-a)).abs()
}

pub fn main() {
    let stones = [31,26,33,21,40].to_vec();
    println!("{}", last_stone_weight_ii(stones));
}
