fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut intervals = intervals
        .into_iter()
        .enumerate()
        .map(|v| (v.1[0], v.1[1], v.1[2], v.0))
        .collect::<Vec<_>>();

    intervals.sort_unstable();

    let n = intervals.len();
    let indices = (0..n).collect::<Vec<_>>();
    let mut dp = vec![(0..4).map(|_| Vec::with_capacity(4)).collect::<Vec<_>>(); n+1];

    let sum = |indices: &[usize]| -> i64 {
        let mut sum = 0;
        for &i in indices {
            sum += intervals[i].2 as i64;
        }

        sum
    };

    let convert = |indices: &[usize]| -> Vec<usize> {
        let mut data = vec![];
        for &i in indices {
            data.push(intervals[i].3);
        }

        data.sort_unstable();
        data
    };

    let process = |dp: &mut Vec<Vec<Vec<usize>>>, at: usize, i: usize, j: usize| {
        if at > 0 { dp[i][at] = dp[j][at-1].clone(); }
        dp[i][at].push(i);

        let a = sum(&dp[i+1][at]);
        let b = sum(&dp[i][at]);
        if (a == b && convert(&dp[i+1][at]) < convert(&dp[i][at])) || a > b {
            dp[i][at] = dp[i+1][at].clone();
        }
    };

    let mut res = vec![];
    for i in (0..n).rev() {
        let j = indices.partition_point(|&j| intervals[i].1 >= intervals[j].0);

        process(&mut dp, 3, i, j);
        process(&mut dp, 2, i, j);
        process(&mut dp, 1, i, j);
        process(&mut dp, 0, i, j);

        for inner in &dp[i] {
            let a = sum(inner);
            let b = sum(&res);
            if (a == b && convert(inner) < convert(&res)) || a > b {
                res = inner.clone();
            }
        }
    }

    let mut res = res.into_iter().map(|i| intervals[i].3 as i32).collect::<Vec<_>>();
    res.sort_unstable();

    res
}

pub fn main() {
    let intervals = [[19,19,46],[23,24,49],[21,25,49],[5,20,42],[11,25,35],[8,24,50],[6,24,4],[11,18,1],[16,18,32],[4,14,16]].into_iter().map(Vec::from).collect();
    println!("{:?}", maximum_weight(intervals));
}
