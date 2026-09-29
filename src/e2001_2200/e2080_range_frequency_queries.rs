use std::collections::HashMap;

#[derive(Debug)]
struct RangeFreqQuery {
    n: usize,
    tree: Vec<HashMap<i32, i32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut init_len = 1;
        while n > init_len {
            init_len <<= 1;
        }

        let mut tree = Self { tree: vec![HashMap::new(); init_len*2-1], n };
        tree.build(&arr, 0, n-1, 0);

        tree
    }

    fn build(&mut self, arr: &[i32], l: usize, r: usize, pos: usize) -> usize {
        if l == r {
            *self.tree[pos].entry(arr[l]).or_default() += 1;
            return pos;
        }

        let m = (l+r)/2;
        let a = self.build(arr, l, m, 2*pos+1);
        let b = self.build(arr, m+1, r, 2*pos+2);

        let mut hashmap = self.tree[a].clone();
        for (&k, &c) in &self.tree[b] {
            hashmap.entry(k).and_modify(|v| *v += c).or_insert(c);
        }

        self.tree[pos] = hashmap;
        pos
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        self.query_frequency(left as usize, right as usize, 0, self.n-1, 0, value)
    }

    fn query_frequency(&self, q_l: usize, q_r: usize, l: usize, r: usize, pos: usize, val: i32) -> i32 {
        if q_l <= l && r <= q_r {
            return *self.tree[pos].get(&val).unwrap_or(&0);
        }

        if q_l > r || q_r < l {
            return 0;
        }

        let m = (l+r)/2;
        self.query_frequency(q_l, q_r, l, m, 2*pos+1, val) + self.query_frequency(q_l, q_r, m+1, r, 2*pos+2, val)
    }
}

pub fn main() {
    let arr = vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56];
    let range_freq_query = RangeFreqQuery::new(arr);
    println!("{:?}", range_freq_query.query(1, 2, 4));
}

