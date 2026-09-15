use std::collections::HashMap;

fn get_max_repetitions(s1: String, mut n1: i32, s2: String, n2: i32) -> i32 {
    let (n, m) = (s1.len(), s2.len());
    let mut hashmap = HashMap::new();
    let mut data = vec![];
    let mut res = 0;
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut at = 0;
    let mut j = 0;
    'outer: while n1 > 0 {
        let mut count = 0;
        for i in 0..n {
            if s1[i] == s2[j] {
                j += 1;
            }

            if j == m {
                count += 1;
                j = 0;
            }

            if let Some(&k) = hashmap.get(&(i, j, count)) {
                at = k;
                break 'outer;
            }

            hashmap.insert((i, j, count), data.len());
        }

        res += count;
        data.push(count);
        n1 -= 1;
    }

    let n = data.len();
    let mut i = at;
    while !data.is_empty() && n1 > 0 {
        n1 -= 1;
        res += data[i];
        i += 1;
        if i == n { i = at; };
    }

    res / n2
}

pub fn main() {
    let s1 = "aaaaaaa".to_string();
    let n1 = 5;
    let s2 = "aaaaaaaa".to_string();
    let n2 = 3;
    println!("{}", get_max_repetitions(s1, n1, s2, n2));
}
