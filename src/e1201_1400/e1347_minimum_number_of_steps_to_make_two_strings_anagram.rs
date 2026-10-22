fn min_steps(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let n = s.len();

    let mut freq_s = [0i32; 26];
    let mut freq_t = [0i32; 26];
    for i in 0..n {
        freq_s[(s[i] - b'a') as usize] += 1;
        freq_t[(t[i] - b'a') as usize] += 1;
    }

    let mut res = 0;
    for i in 0..26 {
        res += (freq_t[i] - freq_s[i]).max(0);
    }

    res
}

pub fn main() {
    let s = "leetcode".to_string();
    let t = "practice".to_string();
    println!("{}", min_steps(s, t));
}
