use std::{cmp::Reverse, collections::BinaryHeap};

fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut min_heap = BinaryHeap::new();
    let mut stack = vec![];
    let mut res = vec![-1; n];
    for i in 0..n {
        while let Some(&Reverse((min, idx))) = min_heap.peek() {
            if min < nums[i] {
                res[idx] = nums[i];
                min_heap.pop();
            } else {
                break;
            }
        }

        while let Some(&last) = stack.last() {
            if nums[last] < nums[i] {
                let idx = stack.pop().unwrap();
                min_heap.push(Reverse((nums[idx], idx)));
            } else {
                break;
            }
        }

        stack.push(i);
    }

    res
}

pub fn main() {
    let nums = [2,4,0,9,6].to_vec();
    println!("{:?}", second_greater_element(nums));
}
