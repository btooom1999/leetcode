fn reverse_vowels(s: String) -> String {
    let is_vowel = |c: char| -> bool {
        ['a', 'i', 'o', 'e', 'u', 'A', 'I', 'O', 'E', 'U'].contains(&c)
    };

    let mut vowels = vec![];
    for c in s.chars() {
        if is_vowel(c) {
            vowels.push(c);
        }
    }

    vowels.reverse();

    let mut i = 0;
    let mut s = s.into_bytes();
    for b in s.iter_mut() {
        if is_vowel(*b as char) {
            *b = vowels[i] as u8;
            i += 1;
        }
    }

    String::from_utf8(s).unwrap()
}

pub fn main() {
    let s = "IceCreAm".to_string();
    println!("{}", reverse_vowels(s));
}
