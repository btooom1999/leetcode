fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
    queries.sort_unstable();

    let (n, m) = (nums.len(), queries.len());
    let mut diff = vec![0; n+1];
    let mut current_diff = 0;
    let mut j = 0;
    let mut max_heap = std::collections::BinaryHeap::new();
    let mut count = 0;

    for i in 0..n {
        current_diff += diff[i];
        while j < m && queries[j][0] <= i as i32 {
            max_heap.push(queries[j][1] as usize);
            j += 1;
        }

        while current_diff < nums[i] {
            if max_heap.peek().is_some_and(|&v| v >= i) {
                diff[max_heap.pop().unwrap()+1] -= 1;
                current_diff += 1;
                count += 1;
            } else {
                return -1;
            }
        }
    }

    (m - count) as i32
}

pub fn main() {
    let nums = [0,0,1,1,0].to_vec();
    let queries = [[3,4],[0,2],[2,3]].into_iter().map(Vec::from).collect();
    println!("{}", max_removal(nums, queries));
}
