fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let n = nums.len();
    let mut diff = vec![0; n+1];
    for query in queries {
        diff[query[0] as usize] -= 1;
        diff[query[1] as usize + 1] += 1;
    }

    for i in 1..n {
        diff[i] += diff[i-1];
    }

    nums.into_iter().enumerate().all(|v| v.1 + diff[v.0] <= 0)
}

pub fn main() {
    let nums = [1,0,1].to_vec();
    let queries = [[0,2]].into_iter().map(Vec::from).collect();
    println!("{}", is_zero_array(nums, queries));
}
