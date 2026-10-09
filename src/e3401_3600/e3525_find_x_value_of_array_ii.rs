fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    let k = 3;
    let queries = [[2,2,0,2],[3,3,3,0],[0,1,0,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", result_array(nums, k, queries));
}
