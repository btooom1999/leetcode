use std::collections::HashMap;

fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }
}

fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut indexes = (0..n).collect::<Vec<_>>();
    indexes.sort_by_key(|&i| nums[i]);

    let mut map = (0..n).collect::<Vec<_>>();
    for i in 1..n {
        if nums[indexes[i]] - nums[indexes[i-1]] <= limit {
            union(&mut map, indexes[i], indexes[i-1]);
        }
    }

    let mut hashmap = HashMap::<_, Vec<usize>>::new();
    for i in 0..n {
        let k = find(&mut map, i);
        hashmap.entry(k).or_default().push(i);
    }

    let mut res = vec![0; n];
    for indexes in hashmap.values() {
        let mut clone = indexes.clone();
        clone.sort_by_key(|&i| nums[i]);

        for i in 0..indexes.len() {
            res[indexes[i]] = nums[clone[i]];
        }
    }

    res
}

pub fn main() {
    let nums = [1,60,34,84,62,56,39,76,49,38].to_vec();
    let limit = 4;
    println!("{:?}", lexicographically_smallest_array(nums, limit));
}
