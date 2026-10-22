fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false; }

    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut flags = [false; 26];
    let mut freq1 = [0; 26];
    let mut freq2 = [0; 26];

    for i in 0..word1.len() {
        freq1[(word1[i]-b'a') as usize] += 1;
        freq2[(word2[i]-b'a') as usize] += 1;
        flags[(word1[i]-b'a') as usize] = true;
        flags[(word2[i]-b'a') as usize] = true;
    }

    freq1.sort_unstable();
    freq2.sort_unstable();

    let flag = flags.into_iter().filter(|v| *v).count() == freq1.into_iter().filter(|&v| v > 0).count();
    flag && freq1 == freq2
}

pub fn main() {
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    println!("{}", close_strings(word1, word2));
}
