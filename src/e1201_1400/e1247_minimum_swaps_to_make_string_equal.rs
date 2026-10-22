fn minimum_swap(s1: String, s2: String) -> i32 {
    let mut a = 0;
    let mut b = 0;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == 'x' && c2 == 'y' {
            a += 1;
        } else if c1 == 'y' && c2 == 'x' {
            b += 1;
        }
    }

    if a % 2 != b % 2 {
        return -1;
    }

    a/2 + b/2 + a%2*2
}

pub fn main() {
    let s1 = "xx".to_string();
    let s2 = "yy".to_string();
    println!("{}", minimum_swap(s1, s2));
}
