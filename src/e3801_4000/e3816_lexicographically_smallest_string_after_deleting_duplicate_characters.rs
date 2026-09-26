fn lex_smallest_after_deletion(s: String) -> String {
    let mut counts = [0; 26];
    let s = s.as_bytes();
    for b in s {
        counts[(b - b'a') as usize] += 1;
    }

    let n = s.len();
    let mut data = vec![];
    for i in 0..n {
        if data.last().is_some_and(|v: &(u8, usize)| v.0 == s[i]) {
            data.last_mut().unwrap().1 += 1;
        } else {
            data.push((s[i], 1));
        }
    }

    let mut stack = vec![];
    let n = data.len();
    for i in 0..n {
        while let Some((prev_b, count)) = stack.last_mut() {
            if *prev_b > data[i].0 {
                let amount = (counts[(*prev_b-b'a') as usize]-1).min(*count);
                counts[(*prev_b-b'a') as usize] -= amount;
                *count -= amount;
                if *count == 0 {
                    stack.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        stack.push(data[i]);
    }

    while let Some((prev_b, count)) = stack.last_mut() {
        let amount = (counts[(*prev_b-b'a') as usize]-1).min(*count);
        counts[(*prev_b-b'a') as usize] -= amount;
        *count -= amount;
        if *count == 0 {
            stack.pop();
        } else {
            break;
        }
    }

    let mut res = vec![];
    for item in stack {
        res.extend(vec![item.0; item.1]);
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let s = "abba".to_string();
    println!("{}", lex_smallest_after_deletion(s));
}
