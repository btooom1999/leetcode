fn min_insertions(s: String) -> i32 {
    let mut open = 0;
    let mut close = 0;
    let mut res = 0;
    for c in s.chars() {
        if c == '(' {
            if close > 0 {
                if open > 0 {
                    open -= 1;
                    res += 1;
                } else {
                    res += 2;
                }
                close = 0;
            }

            open += 1;
        } else if c == ')' {
            close += 1;
            if close == 2 {
                if open == 0 {
                    res += 1;
                } else {
                    open -= 1;
                }

                close = 0;
            }
        }
    }

    if open > 0 && close == 1 {
        res += 1;
        close = 0;
        open -= 1;
    }

    if open > 0 {
        res += open * 2;
    } else if close > 0 {
        res += close / 2 + if close % 2 == 1 { 2 } else { 0 };
    }

    res
}

pub fn main() {
    let s = "))))))((()))(()(()))".to_string();
    println!("{}", min_insertions(s));
}
