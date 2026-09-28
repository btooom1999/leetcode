fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut queries = queries
        .into_iter()
        .enumerate()
        .map(|v| (v.0, v.1[0].min(v.1[1]) as usize, v.1[0].max(v.1[1]) as usize))
        .collect::<Vec<_>>();

    queries.sort_by_key(|v| v.2);

    let n = queries.len();
    let mut stack = vec![];
    let mut res = vec![-1; n];
    for i in (0..n).rev() {
        let (idx, a, b) = (queries[i].0, queries[i].1, queries[i].2);
        let last = *stack.last().unwrap_or(&heights.len());
        for j in (b..last).rev() {
            while let Some(&last) = stack.last() {
                if heights[j] >= heights[last] {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(j);
        }

        if heights[a] < heights[b] || a == b {
            res[idx] = b as i32;
        } else {
            let leftmost = stack.partition_point(|&k| heights[k] > heights[a] && heights[k] > heights[b]);
            if leftmost < stack.len() && heights[stack[leftmost]] > heights[a] && heights[stack[leftmost]] > heights[b] {
                res[idx] = stack[leftmost] as i32;
            } else if leftmost > 0 && heights[stack[leftmost-1]] > heights[a] && heights[stack[leftmost-1]] > heights[b] {
                res[idx] = stack[leftmost-1] as i32;
            }
        }
    }

    res
}

pub fn main() {
    let heights = [1,2,1,2].to_vec();
    let queries = [[0,0],[0,1],[0,2]].into_iter().map(Vec::from).collect();
    // let heights = [3,1,2,4].to_vec();
    // let queries = [[0,0], [0,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", leftmost_building_queries(heights, queries));
}
