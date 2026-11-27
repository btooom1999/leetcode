#[derive(Debug)]
struct Solution {
    prefix: Vec<i32>,
    blacklist: Vec<i32>,
}

impl Solution {
    fn new(n: i32, mut blacklist: Vec<i32>) -> Self {
        blacklist.push(n);
        blacklist.sort_unstable();

        let m = blacklist.len();
        let mut prefix = vec![];
        for i in 0..m {
            let count = blacklist[i] - if i == 0 { -1 } else { blacklist[i-1] } - 1;
            prefix.push(count + *prefix.last().unwrap_or(&0));
        }

        Self { prefix, blacklist }
    }

    fn pick(&self) -> i32 {
        let num = rand::random_range(0..*self.prefix.last().unwrap());
        let i = self.prefix.partition_point(|&v| v <= num);
        let end = self.blacklist[i];
        let start = if i == 0 {
            0
        } else {
            self.blacklist[i-1]+1
        };

        rand::random_range(start..end)
    }
}

pub fn main() {
    let solution = Solution::new(7, vec![0,1,2,3,4,5]);
    println!("{:?}", solution);
    println!("{:?}", solution.pick());
    println!("{:?}", solution.pick());
    println!("{:?}", solution.pick());
    println!("{:?}", solution.pick());
}
