use std::collections::HashMap;

fn minimum_swap(s1: String, s2: String) -> i32 {
    if s1.len() != s2.len() { return -1; }
    let mut hashmap = HashMap::new();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    let mut res = 0;
    for i in 0..s1.len() {
        *hashmap.entry((s1[i], s2[i])).or_default() += 1;
    }

    let a = (b'x', b'y');
    let b = (b'y', b'x');
    let amount = *hashmap.get(&a).unwrap_or(&0) / 2;
    res += amount;
    hashmap.entry(a).and_modify(|v| *v -= amount * 2).or_default();

    let amount = *hashmap.get(&b).unwrap_or(&0) / 2;
    res += amount;
    hashmap.entry(b).and_modify(|v| *v -= amount * 2).or_default();

    let val1 = *hashmap.get(&a).unwrap_or(&0);
    let val2 = *hashmap.get(&b).unwrap_or(&0);
    if val1 != val2 {
        return -1;
    }

    res + val1*2
}

pub fn main() {
    let s1 = "xx".to_string();
    let s2 = "yy".to_string();
    println!("{}", minimum_swap(s1, s2));
}
