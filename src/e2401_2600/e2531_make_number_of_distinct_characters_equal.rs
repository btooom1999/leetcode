fn is_it_possible(word1: String, word2: String) -> bool {
    let mut freq = [(0,0); 26];
    let mut count = (0, 0);
    for c in word1.as_bytes() {
        let k = (c - b'a') as usize;
        freq[k].0 += 1;
        count.0 += (freq[k].0 == 1) as i32;
    }

    for c in word2.as_bytes() {
        let k = (c - b'a') as usize;
        freq[k].1 += 1;
        count.1 += (freq[k].1 == 1) as i32;
    }

    for i in 0..26 {
        if freq[i].0 > 0 {
            for j in 0..26 {
                if freq[j].1 > 0 {
                    freq[i].0 -= 1;
                    freq[j].1 -= 1;
                    count.0 -= (freq[i].0 == 0) as i32;
                    count.1 -= (freq[j].1 == 0) as i32;
                    freq[j].0 += 1;
                    freq[i].1 += 1;
                    count.0 += (freq[j].0 == 1) as i32;
                    count.1 += (freq[i].1 == 1) as i32;
                    if count.0 == count.1 {
                        return true;
                    }
                    freq[i].0 += 1;
                    freq[j].1 += 1;
                    count.0 += (freq[i].0 == 1) as i32;
                    count.1 += (freq[j].1 == 1) as i32;
                    freq[j].0 -= 1;
                    freq[i].1 -= 1;
                    count.0 -= (freq[j].0 == 0) as i32;
                    count.1 -= (freq[i].1 == 0) as i32;
                }
            }
        }

        if freq[i].1 > 0 {
            for j in 0..26 {
                if freq[j].0 > 0 {
                    freq[i].1 -= 1;
                    freq[j].0 -= 1;
                    count.1 -= (freq[i].1 == 0) as i32;
                    count.0 -= (freq[j].0 == 0) as i32;
                    freq[j].1 += 1;
                    freq[i].0 += 1;
                    count.1 += (freq[j].1 == 1) as i32;
                    count.0 += (freq[i].0 == 1) as i32;
                    if count.0 == count.1 {
                        return true;
                    }
                    freq[i].1 += 1;
                    freq[j].0 += 1;
                    count.1 += (freq[i].1 == 1) as i32;
                    count.0 += (freq[j].0 == 1) as i32;
                    freq[j].1 -= 1;
                    freq[i].0 -= 1;
                    count.1 -= (freq[j].1 == 0) as i32;
                    count.0 -= (freq[i].0 == 0) as i32;
                }
            }
        }
    }

    false
}

pub fn main() {
    let word1 = "aa".to_string();
    let word2 = "ab".to_string();
    println!("{}", is_it_possible(word1, word2));
}
