use std::collections::BTreeMap;

fn find(rank: &mut Vec<Option<usize>>, x: usize) -> usize {
    if let Some(next_x) = rank[x] && next_x != x {
        rank[x] = Some(find(rank, next_x));
    }

    rank[x].unwrap()
}

fn union(rank: &mut Vec<Option<usize>>, a: usize, b: usize) {
    let root_a = find(rank, a);
    let root_b = find(rank, b);

    if root_a < root_b {
        rank[root_a] = Some(root_b);
    } else {
        rank[root_b] = Some(root_a);
    }
}

struct SummaryRanges {
    rank: Vec<Option<usize>>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self { rank: vec![None; 10_001] }
    }

    fn add_num(&mut self, value: i32) {
        let value = value as usize;
        self.rank[value] = Some(value);
        if value>0 && self.rank[value-1].is_some() {
            let a = self.rank[value-1].unwrap();
            let b = self.rank[value].unwrap();
            union(&mut self.rank, a, b);
        }
        if value+1<10_001 && self.rank[value+1].is_some() {
            let a = self.rank[value].unwrap();
            let b = self.rank[value+1].unwrap();
            union(&mut self.rank, a, b);
        }
    }

    fn get_intervals(&mut self) -> Vec<Vec<i32>> {
        let mut btreemap = BTreeMap::<usize, Vec<i32>>::new();
        let nums = self.rank.iter().cloned().enumerate().flat_map(|v| {
            if v.1.is_none() { None }
            else { Some(v.0) }
        }).collect::<Vec<_>>();

        for x in nums {
            let root = find(&mut self.rank, x);
            btreemap.entry(root).and_modify(|v| v[1] = x as i32).or_insert(vec![x as i32, x as i32]);
        }

        btreemap.into_values().collect()
    }
}
