fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() == goal.len() {
        let mut chars = [usize::MAX; 26];
        let mut s = s.into_bytes();
        let goal = goal.into_bytes();
        for (i, &b) in s.iter().enumerate() {
            chars[(b - b'a') as usize] = i;
        }

        let n = s.len();
        for i in 0..n {
            let j = chars[(goal[i]-b'a') as usize];
            if j > i && j < usize::MAX {
                (s[i], s[j]) = (s[j], s[i]);
                if s == goal { return true; }
                (s[i], s[j]) = (s[j], s[i]);
            }
        }
    }

    false
}

pub fn main() {
    let s = "ab".to_string();
    let goal = "ba".to_string();
    println!("{}", buddy_strings(s, goal));
}
