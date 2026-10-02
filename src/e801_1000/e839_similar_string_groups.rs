use std::collections::HashSet;

fn find(rank: &mut Vec<usize>, x: usize) -> usize {
    if rank[x] != x {
        rank[x] = find(rank, rank[x]);
    }

    rank[x]
}

fn union(rank: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(rank, a);
    let root_b = find(rank, b);
    if root_a <= root_b {
        rank[root_b] = root_a;
    } else {
        rank[root_a] = root_b;
    }
}

fn check(str1: &mut [u8], str2: &[u8]) -> bool {
    let n = str1.len();
    let mut chars = [usize::MAX; 26];
    for i in 0..n {
        if str1[i] != str2[i] {
            if chars[(str2[i]-b'a') as usize] != usize::MAX {
                let j = chars[(str2[i]-b'a') as usize];
                (str1[i], str1[j]) = (str1[j], str1[i]);
                return str1 == str2;
            }
            chars[(str1[i]-b'a') as usize] = i;
        }
    }

    chars.iter().all(|&v| v == usize::MAX)
}

fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let mut rank = (0..n).collect::<Vec<_>>();

    let mut hashset = HashSet::new();
    for i in 0..n {
        for j in i..n {
            if check(&mut strs[i].clone().into_bytes(), strs[j].as_bytes()) {
                union(&mut rank, i, j);
            }
        }
    }

    for x in 0..n {
        hashset.insert(find(&mut rank, x));
    }

    hashset.len() as i32
}

pub fn main() {
    let strs = ["kccomwcgcs","socgcmcwkc","sgckwcmcoc","coswcmcgkc","cowkccmsgc","cosgmccwkc","sgmkwcccoc","coswmccgkc","kowcccmsgc","kgcomwcccs"].into_iter().map(String::from).collect();
    println!("{}", num_similar_groups(strs));
}
