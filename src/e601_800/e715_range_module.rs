use std::collections::BTreeSet;

#[derive(Debug)]
struct RangeModule {
    btreeset: BTreeSet<(i32, i32)>,
}

impl RangeModule {
    fn new() -> Self {
        Self { btreeset: BTreeSet::new() }
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        let mut new_btreeset = BTreeSet::new();
        for &a in self.btreeset.iter() {
            if a.1 < left || a.0 > right {
                new_btreeset.insert(a);
            } else if (a.0 <= left && left <= a.1) || (a.0 <= right && right <= a.1) {
                left = left.min(a.0);
                right = right.max(a.1);
            }
        }

        new_btreeset.insert((left, right));
        self.btreeset = new_btreeset;
    }

    fn query_range(&self, mut left: i32, right: i32) -> bool {
        for &a in self.btreeset.iter() {
            if a.0 <= left && right <= a.1 {
                return true;
            } else if a.0 <= left && left <= a.1 {
                left = a.1;
            }
        }

        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut new_btreeset = BTreeSet::new();
        for &a in self.btreeset.iter() {
            if a.1 < left || a.0 > right {
                new_btreeset.insert(a);
            } else if a.0 < left && a.1 <= right {
                new_btreeset.insert((a.0, left));
            } else if left <= a.0 && right < a.1 {
                new_btreeset.insert((right, a.1));
            } else if a.0 < left && right < a.1 {
                new_btreeset.insert((a.0, left));
                new_btreeset.insert((right, a.1));
            }
        }

        self.btreeset = new_btreeset;
    }
}

pub fn main() {
    let mut range_module = RangeModule::new();
    range_module.add_range(5, 6);
    range_module.add_range(2, 8);
    range_module.remove_range(2, 4);
    println!("{:?}", range_module);
}
