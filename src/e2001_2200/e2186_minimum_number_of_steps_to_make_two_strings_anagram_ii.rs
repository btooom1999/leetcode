fn min_steps(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut freq_s = [0i32; 26];
    let mut freq_t = [0; 26];
    let (n, m) = (s.len(), t.len());
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        if i < n {
            freq_s[(s[i] - b'a') as usize] += 1;
            i += 1;
        }
        if j < m {
            freq_t[(t[j] - b'a') as usize] += 1;
            j += 1;
        }
    }

    let mut res = 0;
    for i in 0..26 {
        res += (freq_t[i] - freq_s[i]).abs();
    }
    res
}

pub fn main() {
    let s = "leetcode".to_string();
    let t = "coats".to_string();
    println!("{}", min_steps(s, t));
}
