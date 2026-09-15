fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let (l1, l2) = (s1.len(), s2.len());
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut s1_cnt = 0;
    let mut s2_cnt = 0;
    let mut dp = vec![None; l2];
    let mut j = 0;
    while s1_cnt < n1 {
        for i in 0..l1 {
            if s1[i] == s2[j] {
                j += 1;
            }

            if j == l2 {
                s2_cnt += 1;
                j = 0;
            }
        }

        s1_cnt += 1;

        if let Some((prev_s1_cnt, prev_s2_cnt)) = dp[j] {
            let cycle_s1 = s1_cnt - prev_s1_cnt;
            let cycle_s2 = s2_cnt - prev_s2_cnt;

            let remaining_s1 = n1 - s1_cnt;
            let num_cycle= remaining_s1 / cycle_s1;

            s1_cnt += num_cycle * cycle_s1;
            s2_cnt += num_cycle * cycle_s2;
        } else {
            dp[j] = Some((s1_cnt, s2_cnt));
        }
    }

    s2_cnt / n2
}

pub fn main() {
    let s1 = "baba".to_string();
    let n1 = 3;
    let s2 = "ab".to_string();
    let n2 = 1;
    println!("{}", get_max_repetitions(s1, n1, s2, n2));
}
