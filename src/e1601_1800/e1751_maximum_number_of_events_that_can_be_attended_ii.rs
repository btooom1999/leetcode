fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_unstable();

    let n = events.len();
    let starts = events.iter().map(|v| v[0]).collect::<Vec<_>>();
    let next_index = events
        .iter()
        .map(|event| starts.partition_point(|&start| start <= event[1]))
        .collect::<Vec<_>>();

    let mut dp = vec![0; n+1];
    let mut next_dp = vec![0; n+1];
    for _ in 0..k {
        for i in (0..n).rev() {
            next_dp[i] = events[i][2] + dp[next_index[i]];
            next_dp[i] = next_dp[i].max(next_dp[i+1]);
        }

        std::mem::swap(&mut dp, &mut next_dp);
    }

    dp[0]
}

pub fn main() {
    let events = [[1,2,4],[3,4,3],[2,3,1]].into_iter().map(Vec::from).collect();
    let k = 2;
    println!("{}", max_value(events, k));
}
