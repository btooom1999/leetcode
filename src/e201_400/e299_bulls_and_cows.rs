fn get_hint(secret: String, guess: String) -> String {
    let secret = secret.as_bytes();
    let guess = guess.as_bytes();
    let n = secret.len();
    let mut bulls = 0;
    let mut cows = 0;
    let mut counts = [0; 10];
    for i in 0..n {
        if secret[i] != guess[i] {
            counts[(secret[i]-b'0') as usize] += 1;
        } else {
            bulls += 1;
        }
    }

    for i in 0..n {
        if secret[i] != guess[i] && counts[(guess[i]-b'0') as usize] > 0 {
            cows += 1;
            counts[(guess[i]-b'0') as usize] -= 1;
        }
    }

    format!("{bulls}A{cows}B")
}

pub fn main() {
    let secret = "1123".to_string();
    let guess = "0111".to_string();
    println!("{}", get_hint(secret, guess));
}
