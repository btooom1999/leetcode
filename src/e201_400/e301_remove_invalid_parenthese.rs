use std::collections::HashSet;

fn backtracking(
    s: &mut [u8],
    i: usize,
    res: &mut HashSet<String>,
    removal_count: i32,
    removal_min: &mut i32,
) {
    if i == s.len() {
        let mut parentheses = 0;
        let mut str = String::new();
        for i in 0..s.len() {
            if s[i] == b'(' {
                parentheses += 1;
            } else if s[i] == b')' {
                if parentheses == 0 { return; }
                parentheses -= 1;
            }

            if s[i] != b'*' {
                str.push(s[i] as char);
            }
        }

        if parentheses == 0 {
            if removal_count < *removal_min {
                *removal_min = removal_count;
                *res = HashSet::from([str]);
            } else if removal_count == *removal_min {
                res.insert(str);
            }
        }
    } else {
        if removal_count <= * removal_min {
            backtracking(s, i+1, res, removal_count, removal_min);
        }

        if removal_count < *removal_min && !s[i].is_ascii_lowercase() {
            let prev = s[i];
            s[i] = b'*';
            backtracking(s, i+1, res, removal_count+1, removal_min);
            s[i] = prev;
        }
    }
}

fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut res = HashSet::new();
    backtracking(&mut s.into_bytes(), 0, &mut res, 0, &mut 50);
    res.into_iter().collect::<Vec<_>>()
}

pub fn main() {
    let s = "(a)())()".to_string();
    println!("{:?}", remove_invalid_parentheses(s));
}
