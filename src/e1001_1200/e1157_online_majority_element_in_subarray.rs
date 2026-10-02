use std::collections::HashMap;

struct MajorityChecker {
    tree: Vec<HashMap<i32, i32>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut tree = vec![HashMap::new(); n+1];
        for i in 0..n {
            let mut k = i+1;
            while k <= n {
                *tree[k].entry(arr[i]).or_default() += 1;
                k += k & !(k-1);
            }
        }

        Self { tree }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let a = self.get_prefix((left+1) as usize);
        let b = self.get_prefix((right+1) as usize);
        let mut max = (0, 0);
        for (k, c) in b {
            let val = c - a.get(&k).unwrap_or(&0);
            if val > max.1 {
                max = (k, val);
            }
        }

        if max.1 >= threshold {
            max.0
        } else {
            -1
        }
    }

    fn get_prefix(&self, mut k: usize) -> HashMap<i32, i32> {
        let mut hashmap = HashMap::new();
        while k > 0 {
            for (&k, &c) in &self.tree[k] {
                hashmap.entry(k).and_modify(|v| *v += c).or_insert(c);
            }
            k -= k & !(k-1);
        }

        hashmap
    }
}

pub fn main() {

}
