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

fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let s = s.as_bytes();
    let n = s.len();
    let mut map = (0..n).collect::<Vec<_>>();
    for pair in pairs {
        union(&mut map, pair[0] as usize, pair[1] as usize);
    }

    let mut hashmap = HashMap::<_, Vec<usize>>::new();
    for i in 0..n {
        let x = map[i];
        hashmap.entry(find(&mut map, x)).or_default().push(i);
    }

    let mut res = s.to_vec();
    for indexes in hashmap.values() {
        let mut clone = indexes.clone();
        clone.sort_by_key(|&v| s[v]);

        for i in 0..indexes.len() {
            res[indexes[i]] = s[clone[i]];
        }
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let s = "pwqlmqm".to_string();
    let pairs = [[5,3],[3,0],[5,1],[1,1],[1,5],[3,0],[0,2]].into_iter().map(Vec::from).collect();
    println!("{}", smallest_string_with_swaps(s, pairs));
}

#[cfg(test)]
mod test {
    use crate::e1201_1400::e1202_smallest_string_with_swaps::smallest_string_with_swaps;

    #[test]
    fn case_1() {
        let s = "dcab".to_string();
        let pairs = [[0,3],[1,2]].into_iter().map(Vec::from).collect();

        assert!(smallest_string_with_swaps(s, pairs) == "bacd")
    }

    #[test]
    fn case_2() {
        let s = "pwqlmqm".to_string();
        let pairs = [[5,3],[3,0],[5,1],[1,1],[1,5],[3,0],[0,2]].into_iter().map(Vec::from).collect();

        assert!(smallest_string_with_swaps(s, pairs) == "lpqqmwm")
    }
}
