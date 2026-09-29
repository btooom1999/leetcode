fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

pub fn main() {
    let nums1 = [4,3,1,2].to_vec();
    let nums2 = [2,4,9,5].to_vec();
    let queries = [[4,1],[1,3],[2,5]].into_iter().map(Vec::from).collect();
    println!("{:?}", maximum_sum_queries(nums1, nums2, queries));
}
