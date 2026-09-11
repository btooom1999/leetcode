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

fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
    let n = source.len();
    let mut map = (0..n).collect::<Vec<_>>();
    for swap in allowed_swaps {
        union(&mut map, swap[0] as usize, swap[1] as usize);
    }

    let mut hashmap = HashMap::<_, HashMap<i32, i32>>::new();
    for i in 0..n {
        let k = find(&mut map, i);
        *hashmap.entry(k).or_default().entry(target[i]).or_default() += 1;
    }

    let mut res = n;
    for i in 0..n {
        let k = find(&mut map, i);
        if let Some(val) = hashmap.get_mut(&k).unwrap().get_mut(&source[i]) && *val > 0 {
            *val -= 1;
            res -= 1;
        }
    }

    res as i32
}

pub fn main() {
    let source = [1,2,3,4].to_vec();
    let target = [2,1,4,5].to_vec();
    let allowed_swaps = [[0,1],[2,3]].into_iter().map(Vec::from).collect();
    println!("{}", minimum_hamming_distance(source, target, allowed_swaps));
}
