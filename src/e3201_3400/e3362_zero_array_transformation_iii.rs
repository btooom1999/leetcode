fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    todo!()
}

pub fn main() {
    let nums = [2,0,2].to_vec();
    let queries = [[0,2],[0,2],[1,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_removal(nums, queries));
}
