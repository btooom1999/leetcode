use std::collections::{BinaryHeap, BTreeSet};
use std::cmp::Reverse;

struct ExamRoom {
    btreeset: BTreeSet<i32>,
    n: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self { btreeset: BTreeSet::new(), n: n-1 }
    }

    fn seat(&mut self) -> i32 {
        if self.btreeset.is_empty() {
            self.btreeset.insert(0);
            return 0;
        }

        let mut heap = BinaryHeap::new();
        if let Some(&first) = self.btreeset.first() && first != 0 {
            heap.push((first, Reverse(0)));
        }

        if let Some(&last) = self.btreeset.last() && last != self.n {
            heap.push((self.n-last, Reverse(self.n)));
        }

        let mut prev = *self.btreeset.first().unwrap();

        for &k in self.btreeset.iter().skip(1) {
            let m = (k+prev)/2;
            heap.push(((m-prev).min(k-m), Reverse(m)));
            prev = k;
        }
        let ans = heap.pop().unwrap().1.0;
        self.btreeset.insert(ans);

        ans
    }

    fn leave(&mut self, p: i32) {
        self.btreeset.remove(&p);
    }
}

pub fn main() {}
