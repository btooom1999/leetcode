fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut stack = Vec::<i32>::new();
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        while let Some(&last) = stack.last() {
            res[i] += 1;
            if last < heights[i] {
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(heights[i]);
    }

    res
}

pub fn main() {
    let heights = [10,6,8,5,11,9].to_vec();
    println!("{:?}", can_see_persons_count(heights));
}
