fn reverse_only_letters(s: String) -> String {
    let mut s = s.into_bytes();
    let mut l = 0;
    let mut r = s.len()-1;
    while l < r {
        while l < r && !s[l].is_ascii_alphabetic() {
            l += 1;
        }

        while l < r && !s[r].is_ascii_alphabetic() {
            r -= 1;
        }

        if l >= r { break; }

        (s[l], s[r]) = (s[r], s[l]);
        l += 1;
        r -= 1;
    }

    String::from_utf8(s).unwrap()
}

pub fn main() {
    let s = "Test1ng-Leet=code-Q!".to_string();
    println!("{}", reverse_only_letters(s));
}
