fn z_algorithm(p: &[u8], s: &[u8]) -> Vec<usize> {
    let str = format!("{}#{}", String::from_utf8(p.to_vec()).unwrap(), String::from_utf8(s.to_vec()).unwrap());
    let str = str.as_bytes();

    let n = str.len();
    let mut z = vec![0; n];
    let mut left = 0;
    let mut right = 0;
    for k in 1..n {
        let k1 = k - left;
        if k > right || z[k1] > right-k {
            left = k;
            right = right.max(k);
            while right < n && str[right] == str[right-left] {
                right += 1;
            }

            z[k] = right-left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }
    }

    z[p.len()+1..].to_vec()
}

fn generate_string(str1: String, str2: String) -> String {
    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();
    let (n, m) = (str1.len(), str2.len());

    let mut s = vec![b'_'; n+m-1];
    for i in 0..n {
        if str1[i] == b'T' {
            for j in 0..m {
                if s[i+j] != b'_' && s[i+j] != str2[j] {
                    return String::new()
                }
                s[i+j] = str2[j];
            }
        }
    }

    for i in 0..n+m-1 {
        if s[i] == b'_' {
            s[i] = b'a';
        }
    }

    let mut z = z_algorithm(str2, &s);
    println!("before {:?}", z);
    let mut at = z.len()-1;
    for i in (0..n).rev() {
        if str1[i] == b'T' {
            if z[i] != m {
                return String::new();
            }

            at = i.saturating_sub(1);
        } else if str1[i] == b'F' && z[i] == m {
            for k in (at.saturating_sub(m-1)..=at).rev() {
                z[k] -= 1;
            }
            s[at] = b'b';
            at = i.saturating_sub(1);
        }

        println!("{:?} {} {}", z, i, String::from_utf8(s.to_vec()).unwrap());
    }

    String::from_utf8(s.to_vec()).unwrap()
}

pub fn main() {
    let str1 = "TFFF".to_string();
    let str2 = "aaa".to_string();
    println!("{}", generate_string(str1, str2));
}
