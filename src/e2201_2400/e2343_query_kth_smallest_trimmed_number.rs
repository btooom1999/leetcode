use std::{cmp::Reverse, collections::BinaryHeap};

fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let nums = nums.into_iter().map(|v| v.into_bytes()).collect::<Vec<_>>();
    let mut queries = queries.into_iter().enumerate().map(|v| (v.1[0], v.1[1] as usize, v.0)).collect::<Vec<_>>();
    queries.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut radix = vec![BinaryHeap::new(); 10];
    let (n, m) = (nums.len(), nums[0].len());
    for i in 0..n {
        radix[0].push(Reverse((String::new(), i)));
    }

    let mut res = vec![-1; queries.len()];
    let mut k = 0;
    for at in 0..=m {
        let mut new_radix = vec![BinaryHeap::new(); 10];
        let mut idx = 1;
        for j in 0..10 {
            while let Some(Reverse((suffix, i))) = radix[j].pop() {
                while k < queries.len() && queries[k].0 == idx && queries[k].1 == at {
                    res[queries[k].2] = i as i32;
                    k += 1;
                }

                idx += 1;
                if at < m {
                    let str = format!("{}{suffix}", nums[i][m-at-1] as char);
                    new_radix[(nums[i][m-at-1]-b'0') as usize].push(Reverse((str, i)));
                }
            }
        }

        radix = new_radix;
    }

    res
}

pub fn main() {
    let nums = ["64333639502","65953866768","17845691654","87148775908","58954177897","70439926174","48059986638","47548857440","18418180516","06364956881","01866627626","36824890579","14672385151","71207752868"].into_iter().map(String::from).collect();
    let queries = [[9,4],[6,1],[3,8],[12,9],[11,4],[4,9],[2,7],[10,3],[13,1],[13,1],[6,1],[5,10]].into_iter().map(Vec::from).collect();
    println!("{:?}", smallest_trimmed_numbers(nums, queries));
}
