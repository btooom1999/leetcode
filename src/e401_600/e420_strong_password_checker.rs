use std::{cmp::Reverse, collections::BinaryHeap};

fn strong_password_checker(password: String) -> i32 {
    let mut password = password.into_bytes();
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;

    for b in &password {
        has_uppercase = has_uppercase || b.is_ascii_uppercase();
        has_lowercase = has_lowercase || b.is_ascii_lowercase();
        has_digit = has_digit || b.is_ascii_digit();
    }

    let mut res = 0;
    if password.len() < 6 {
        let special_chars = [b'!', b'@', b'#', b'$', b'%', b'^', b'&'];
        let mut i = 1;
        while i < password.len() && password.len() < 6 {
            if password[i-1] == password[i] && password[i] == password[i+1] {
                password.insert(i+1, b'#');
                res += 1;
            } else {
                i += 1;
            }
        }
        while password.len() < 6 {
            password.push(special_chars[password.len()]);
            res += 1;
        }

        for b in password.iter_mut() {
            if special_chars.contains(b) {
                if !has_digit {
                    *b = b'0';
                    has_digit = true;
                } else if !has_lowercase {
                    *b = b'a';
                    has_lowercase = true;
                } else if !has_uppercase {
                    *b = b'A';
                    has_uppercase = true;
                }
            }
        }
    }

    let mut cur = password[0];
    let mut count = 0;
    let mut heap = BinaryHeap::new();

    let n = password.len();
    for i in 0..n {
        if cur == password[i] {
            count += 1;
        } else {
            if count > 2 {
                heap.push(Reverse((count % 3, count)));
            }
            cur = password[i];
            count = 1;
        }

        if i == n-1 && count > 2 {
            heap.push(Reverse((count % 3, count)));
        }
    }

    let mut amount = (n as i32 - 20).max(0);
    while amount > 0 && let Some(Reverse(min)) = heap.pop() {
        if min.1 < 3 {
            heap.push(Reverse((999, min.1)));
            if min.0 == 999 { break; }
        } else {
            let k = (min.0+1).min(amount);
            let b = min.1 - k;
            let a = b % 3;
            res += k;
            amount -= k;
            heap.push(Reverse((a, b)));
        }
    }

    while let Some(Reverse(min)) = heap.pop() {
        let mut val = min.1 / 3;
        if !has_lowercase {
            has_lowercase = true;
            res += 1;
            val -= 1;
        }
        if val > 0 && !has_uppercase {
            has_uppercase = true;
            res += 1;
            val -= 1;
        }
        if val > 0 && !has_digit {
            has_digit = true;
            res += 1;
            val -= 1;
        }
        res += val;
    }

    res + amount + (!has_uppercase) as i32 + (!has_lowercase as i32) + (!has_digit) as i32
}

pub fn main() {
    let password = "aaabbb".to_string();
    println!("{}", strong_password_checker(password));
}
