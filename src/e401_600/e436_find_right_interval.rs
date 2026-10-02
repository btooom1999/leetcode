fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut intervals = intervals.into_iter().enumerate().collect::<Vec<_>>();
    intervals.sort_by_key(|v| v.1[0]);
    let n = intervals.len();
    let mut res = vec![-1; n];
    for (i, interval) in &intervals {
        let k = intervals.partition_point(|v| v.1[0] < interval[1]);
        if k != n {
            res[*i] = intervals[k].0 as i32;
        }
    }

    res
}

pub fn main() {
    let intervals = [[3,4],[2,3],[1,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_right_interval(intervals));
}
