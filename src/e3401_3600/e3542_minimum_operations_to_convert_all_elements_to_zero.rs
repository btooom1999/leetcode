fn min_operations(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut stack = vec![];
    for num in nums {
        while stack.last().is_some_and(|v| *v > num) {
            count += 1;
            stack.pop();
        }

        if num != 0 && stack.last().is_none_or(|v| *v != num) {
            stack.push(num);
        }

    }

    count + stack.len() as i32
}

pub fn main() {
    let nums = [1,2,1,2,1,2].to_vec();
    println!("{}", min_operations(nums));
}
