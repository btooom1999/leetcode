fn max_taxi_earnings(_n: i32, mut rides: Vec<Vec<i32>>) -> i64 {
    rides.sort_unstable();

    let n = rides.len();
    let mut dp = vec![0; n+1];
    let indices = (0..n).collect::<Vec<_>>();
    for i in (0..n).rev() {
        let j = indices.partition_point(|&j| rides[i][1] > rides[j][0]);
        dp[i] = dp[j] + rides[i][1] as i64 - rides[i][0] as i64 + rides[i][2] as i64;
        dp[i] = dp[i].max(dp[i+1]);
    }

    dp[0]
}

pub fn main() {
    let n = 20;
    let rides = [[1,6,1],[3,10,2],[10,12,3],[11,12,2],[12,15,2],[13,18,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_taxi_earnings(n, rides));
}
