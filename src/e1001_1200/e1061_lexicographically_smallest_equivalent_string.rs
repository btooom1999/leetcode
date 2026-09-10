fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut chars = [27; 26];
    let mut map = vec![vec![]; 26];
    let n = s1.len();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    for i in 0..n {
        map[(s1[i] - b'a') as usize].push((s2[i] - b'a') as usize);
        map[(s2[i] - b'a') as usize].push((s1[i] - b'a') as usize);
    }

    for i in 0..26 {
        let mut q = std::collections::VecDeque::from([i]);
        while let Some(j) = q.pop_front() {
            if !map[j].is_empty() && chars[j] > i {
                chars[j] = i;
                for &i_next in &map[j] {
                    q.push_back(i_next);
                }
            }
        }
    }

    let mut base_str = base_str.into_bytes();
    for i in 0..base_str.len() {
        let b = chars[(base_str[i] - b'a') as usize] as u8 + b'a';
        if  base_str[i] > b {
            base_str[i] = b;
        }
    }

    String::from_utf8(base_str).unwrap()
}

pub fn main() {
    let s1 = "parker".to_string();
    let s2 = "morris".to_string();
    let base_str = "parser".to_string();
    println!("{}", smallest_equivalent_string(s1, s2, base_str));
}
