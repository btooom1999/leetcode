fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut data = start_time
        .into_iter()
        .zip(end_time)
        .zip(profit)
        .map(|v| (v.0.0, v.0.1, v.1))
        .collect::<Vec<_>>();

    data.sort_unstable();

    let n = data.len();
    let mut dp = vec![0; n+1];
    let indices = (0..n).collect::<Vec<_>>();

    for i in (0..n).rev() {
        let j = indices.partition_point(|&j| data[i].1 > data[j].0);
        dp[i] = dp[i+1].max(data[i].2 + dp[j]);
    }

    dp[0]
}

pub fn main() {
    let start_time = [1,2,3,3].to_vec();
    let end_time = [3,4,5,6].to_vec();
    let profit = [50,10,40,70].to_vec();
    println!("{}", job_scheduling(start_time, end_time, profit));
}
