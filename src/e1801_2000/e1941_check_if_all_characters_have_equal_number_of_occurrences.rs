fn are_occurrences_equal(s: String) -> bool {
    let mut freq = [0; 26];
    let mut max = 0;
    for b in s.as_bytes() {
        freq[(b-b'a') as usize] += 1;
        max = max.max(freq[(b-b'a') as usize]);
    }

    freq.into_iter().all(|v| v == 0 || v == max)
}

pub fn main() {
    let s = "abacbc".to_string();
    println!("{}", are_occurrences_equal(s));
}
