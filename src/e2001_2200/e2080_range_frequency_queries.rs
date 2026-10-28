use std::collections::HashMap;

#[derive(Debug)]
struct RangeFreqQuery {
    arr: Vec<i32>,
    blocks: Vec<HashMap<i32, i32>>,
    block_size: usize,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let block_size = (n as f32).sqrt().ceil() as usize;
        let mut blocks = vec![HashMap::new(); (n+block_size-1)/block_size];
        for (i, &num) in arr.iter().enumerate() {
            *blocks[i/block_size].entry(num).or_default() += 1;
        }

        Self { arr, blocks, block_size }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let l = left / self.block_size;
        let r = right / self.block_size;
        let mut count = 0;
        if l == r {
            for i in left..=right {
                count += (self.arr[i] == value) as i32;
            }
        } else {
            for i in left..(l+1)*self.block_size {
                count += (self.arr[i] == value) as i32;
            }

            for i in l+1..r {
                count += *self.blocks[i].get(&value).unwrap_or(&0);
            }

            for i in r*self.block_size..right+1 {
                count += (self.arr[i] == value) as i32;
            }
        }

        count
    }
}

pub fn main() {
    let arr = [12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56].to_vec();
    let range_freq_query = RangeFreqQuery::new(arr);
    // print!("{:?}", range_freq_query);
    println!("{:?}", range_freq_query.query(0, 11, 33));
}

