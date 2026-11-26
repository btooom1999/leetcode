struct Solution {
    counts: Vec<i64>,
    rects: Vec<Vec<i32>>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut counts = vec![];
        for rect in &rects {
            counts.push((rect[2]-rect[0]+1) as i64 * (rect[3]-rect[1]+1) as i64 + *counts.last().unwrap_or(&0));
        }

        Self { rects, counts }
    }

    fn pick(&self) -> Vec<i32> {
        let num = rand::random_range(0..=*self.counts.last().unwrap());
        let i = self.counts.partition_point(|&v| v < num);
        vec![
            rand::random_range(self.rects[i][0]..=self.rects[i][2]),
            rand::random_range(self.rects[i][1]..=self.rects[i][3]),
        ]
    }
}

pub fn main() {}
