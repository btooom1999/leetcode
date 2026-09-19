fn total_steps(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut stack = vec![];
    for num in nums {
        let mut count = 0;
        while stack.last().is_some_and(|(val, _)| *val <= num) {
            count = count.max(stack.pop().unwrap().1);
        }

        if stack.is_empty() {
            count = 0;
        } else {
            count += 1;
        }

        max = max.max(count);
        stack.push((num, count));
    }

    max
}

pub fn main() {
    let nums = [5,3,5,4,4,7,3,6,11,8,5,11].to_vec();
    println!("{}", total_steps(nums));
}
