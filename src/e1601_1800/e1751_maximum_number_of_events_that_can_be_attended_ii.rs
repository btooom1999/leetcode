use std::collections::BTreeSet;

fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_unstable();

    let k = k as usize;
    let n = events.len();
    let mut dp = vec![(0, BTreeSet::new()); n+1];
    let indices = (0..n).collect::<Vec<_>>();
    let mut res = 0;

    for i in (0..n).rev() {
        let j = indices.partition_point(|&j| events[i][1] >= events[j][0]);
        dp[i] = dp[j].clone();
        dp[i].0 += events[i][2];
        dp[i].1.insert((events[i][2], i));

        if dp[i].1.len() > k {
            dp[i].0 -= dp[i].1.pop_first().unwrap().0;
        }

        if dp[i+1].0 > dp[i].0 {
            dp[i] = dp[i+1].clone();
        }

        res = res.max(dp[i].0);
    }

    res
}

pub fn main() {
    let events = [[1,2,4],[3,4,3],[2,3,1]].into_iter().map(Vec::from).collect();
    let k = 2;
    println!("{}", max_value(events, k));
}
