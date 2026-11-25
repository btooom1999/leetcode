fn final_string(s: String) -> String {
    let n = s.len();
    let s = s.as_bytes();
    let mut ans = vec![];
    for j in 0..n {
        if s[j] == b'i' {
            ans.reverse();
        } else {
            ans.push(s[j]);
        }
    }

    String::from_utf8(ans).unwrap()
}

pub fn main() {
    let s = "string".to_string();
    println!("{}", final_string(s));
}
