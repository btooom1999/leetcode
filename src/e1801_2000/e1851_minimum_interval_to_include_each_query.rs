fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    todo!()
}

pub fn main() {
    let intervals = [[1,4],[2,4],[3,6],[4,4]].into_iter().map(Vec::from).collect();
    let queries = [2,3,4,5].to_vec();
    println!("{:?}", min_interval(intervals, queries));
}
