use std::collections::HashMap;

struct MajorityChecker {
    n: usize,
    tree: Vec<(i32, i32)>,
    hashmap: HashMap<i32, Vec<usize>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut tree_n = 1;
        while tree_n < arr.len() {
            tree_n <<= 1;
        }

        let mut data = Self { tree: vec![(-1,0); 2*tree_n-1], n: tree_n-1, hashmap: HashMap::new() };
        let bias = tree_n-1;

        for i in 0..n {
            data.tree[i+bias] = (arr[i],1);
            data.hashmap.entry(arr[i]).or_default().push(i);
        }

        for i in (0..bias).rev() {
            data.tree[i] = data.choose(data.tree[i*2+1], data.tree[i*2+2]);
        }

        data
    }

    fn choose(&self, left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
        if left.0 == -1 {
            right
        } else if right.0 == -1 {
            left
        } else if left.0 == right.0 {
            (left.0, left.1+right.1)
        } else if left.1 > right.1 {
            (left.0, left.1-right.1)
        } else if left.1 < right.1 {
            (right.0, right.1-left.1)
        } else {
            (left.0, 0)
        }
    }

    fn query_tree(&self, l: usize, h: usize, ql: usize, qh: usize, pos: usize) -> (i32, i32) {
        if h < ql || l > qh {
            return (-1, 0);
        }

        if ql <= l && h <= qh {
            return self.tree[pos];
        }

        let m = (l+h)/2;
        let left = self.query_tree(l, m, ql, qh, pos*2+1);
        let right = self.query_tree(m+1, h, ql, qh, pos*2+2);
        self.choose(left, right)
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let res = self.query_tree(0, self.n-1, left, right, 0);
        if let Some(pos) = self.hashmap.get(&res.0) {
            let l = pos.partition_point(|&i| i < left);
            let r = pos.partition_point(|&i| i <= right);
            if (r-l) as i32 >= threshold { res.0 } else { -1 }
        } else {
            -1
        }
    }
}

pub fn main() {
    let majority_checker = MajorityChecker::new(vec![1,1,1,1,1,2,2,2,2,2,2,1,1,1,1,1]);
    majority_checker.query(3, 12, 6);
}
