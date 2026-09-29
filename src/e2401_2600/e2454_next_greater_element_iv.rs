use std::{cmp::Reverse, collections::BinaryHeap};

fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut min_heap = BinaryHeap::new();
    let mut stack = vec![];
    let mut res = vec![-1; n];
    for i in 0..n {
        let mut elements = vec![];
        while let Some(&last) = stack.last() {
            if nums[last] < nums[i] {
                elements.push(stack.pop().unwrap());
            } else {
                break;
            }
        }

        while let Some(&Reverse((min, idx))) = min_heap.peek() {
            if min < nums[i] {
                res[idx] = nums[i];
                min_heap.pop();
            } else {
                break;
            }
        }

        for el in elements {
            min_heap.push(Reverse((nums[el], el)));
        }

        stack.push(i);
    }

    res
}

// [1,17,18,0,18,10,20,0]

// heap = [17,18]
// temp = [18,10]
// [20]

pub fn main() {
    let nums = [2,4,0,9,6].to_vec();
    println!("{:?}", second_greater_element(nums));
}
