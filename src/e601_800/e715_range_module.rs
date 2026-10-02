use std::cell::Cell;
use std::collections::BTreeSet;

#[derive(Debug)]
struct RangeModule {
    btreeset: BTreeSet<Cell<(i32, i32)>>,
}

impl RangeModule {
    fn new() -> Self {
        Self { btreeset: BTreeSet::new() }
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        self.btreeset.retain(|v| {
            let v = v.get();
            if (v.0 <= left && left <= v.1) || (v.0 <= right && right <= v.1) {
                left = left.min(v.0);
                right = right.max(v.1);
                return false;
            } else if v.1 < left || v.0 > right {
                return true;
            }

            false
        });

        self.btreeset.insert(Cell::new((left, right)));
    }

    fn query_range(&self, mut left: i32, right: i32) -> bool {
        for a in self.btreeset.iter() {
            let a = a.get();
            if a.0 <= left && right <= a.1 {
                return true;
            } else if a.0 <= left && left <= a.1 {
                left = a.1;
            }
        }

        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut need = vec![];
        self.btreeset.retain(|a| {
            let v = a.get();
            if v.1 < left || v.0 > right {
                true
            } else if v.0 < left && v.1 <= right {
                a.set((v.0, left));
                true
            } else if left <= v.0 && right < v.1 {
                a.set((right, v.1));
                true
            } else if v.0 < left && right < v.1 {
                a.set((v.0, left));
                need.push((right, v.1));
                true
            } else {
                false
            }
        });

        while let Some(key) = need.pop() {
            self.btreeset.insert(Cell::new(key));
        }
    }
}

pub fn main() {
    let mut range_module = RangeModule::new();
    range_module.add_range(5, 6);
    range_module.add_range(2, 8);
    range_module.remove_range(2, 4);
    println!("{:?}", range_module);
}
