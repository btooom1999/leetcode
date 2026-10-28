fn check(nums: &[i32], queries: &[Vec<i32>]) -> bool {
    let mut diff = vec![0; nums.len()+1];
    for query in queries {
        diff[query[0] as usize] -= query[2];
        diff[query[1] as usize + 1] += query[2];
    }

    for i in 1..diff.len() {
        diff[i] += diff[i-1];
    }

    nums.iter().enumerate().all(|v| v.1 + diff[v.0] <= 0)
}

fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut l = 0;
    let mut r = queries.len()+1;

    while l < r {
        let m = (l+r)/2;
        if check(&nums, &queries[..m]) {
            r = m;
        } else {
            l = m+1;
        }
    }

    if l == queries.len()+1 { -1 } else { l as i32 }
}

pub fn main() {
    let nums = [2].to_vec();
    let queries = [[0,0,6],[0,0,2],[0,0,9],[0,0,5],[0,0,10]].into_iter().map(Vec::from).collect();
    println!("{}", min_zero_array(nums, queries));
}
