fn make_string_good(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut freq = [0; 26];
    for i in 0..n {
        freq[(s[i]-b'a') as usize] += 1;
    }

    let mut res = i32::MAX;
    for k in (1..=n as i32).rev() {
        let mut dp = [i32::MAX; 27];
        dp[26] = 0;
        for i in (0..26).rev() {
            if freq[i] == 0 {
                dp[i] = dp[i+1];
                continue;
            }

            if freq[i] <= k {
                dp[i] = (k - freq[i]).min(freq[i]) + dp[i+1];
            } else {
                dp[i] = freq[i] - k + dp[i+1];
            }

            if i+1 < 26 && freq[i+1] > 0 && freq[i+1] < k {
                if freq[i] > k {
                    let amount = freq[i] - k;
                    let extra = (k - (freq[i+1] + amount)).max(0);
                    dp[i] = dp[i].min(amount + extra + dp[i+2]);
                } else {
                    let need = k - freq[i+1];
                    let remaining = (freq[i] - need).max(0);
                    dp[i] = dp[i].min((k-remaining).min(remaining) + need + dp[i+2]);
                }
            }
        }

        res = res.min(dp[0]);
    }

    res
}

pub fn main() {
    // let s = "kkkkkkkllttttt".to_string();
    let s = "sssss".to_string();
    println!("{}", make_string_good(s));
}
