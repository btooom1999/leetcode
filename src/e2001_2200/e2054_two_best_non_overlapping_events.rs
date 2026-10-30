fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    events.sort_unstable();

    let n = events.len();
    let mut dp = vec![0; n+1];
    let indices = (0..n).collect::<Vec<_>>();

    let mut res = 0;
    for i in (0..n).rev() {
        let j = indices.partition_point(|&j| events[i][1] >= events[j][0]);
        dp[i] = events[i][2].max(dp[i+1]);
        res = res.max(events[i][2] + dp[j]);
    }

    res
}

pub fn main() {
    let events = [[1,3,2],[4,5,2],[2,4,3]].into_iter().map(Vec::from).collect();
    println!("{}", max_two_events(events));
}
