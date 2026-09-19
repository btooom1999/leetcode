fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
    let n = heights.len();
    let mut stack = vec![];
    let mut left = vec![0; n];
    for i in 0..n {
        let mut count = 0;
        while stack.last().is_some_and(|&(prev_i, _)| heights[prev_i] > heights[i]) {
            count += stack.pop().unwrap().1;
        }

        left[i] = heights[i] as i64 * count + stack.last().map_or(0, |&(prev_i, _)| left[prev_i] + heights[prev_i] as i64);
        stack.push((i, count+1));
    }

    stack.clear();
    let mut res = 0;
    let mut right = vec![0; n];
    for i in (0..n).rev() {
        let mut count = 0;
        while stack.last().is_some_and(|&(prev_i, _)| heights[prev_i] > heights[i]) {
            count += stack.pop().unwrap().1;
        }

        right[i] = heights[i] as i64 * count + stack.last().map_or(0, |&(prev_i, _)| right[prev_i] + heights[prev_i] as i64);
        res = res.max(left[i] + right[i] + heights[i] as i64);
        stack.push((i, count+1));
    }

    res
}

pub fn main() {
    let heights = [5,3,4,1,1].to_vec();
    println!("{}", maximum_sum_of_heights(heights));
}
