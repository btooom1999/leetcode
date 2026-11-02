fn backtracking1(
    flags: &mut [bool],
    n: i32,
    num: i32
) -> i32 {
    if num > n {
        return num;
    }

    let mut res = i32::MAX;
    for i in 1..10 {
        if flags[i] { continue; }
        let mut num = num;
        for _ in 0..i {
            num = num.saturating_mul(10);
            num = num.saturating_add(i as i32);
        }

        flags[i] = true;
        let min = backtracking1(flags, n, num);
        res = res.min(min);
        flags[i] = false;
    }

    res
}

fn backtracking2(
    counts: &mut [i32],
    n: i32,
    num: i32,
) -> i32 {
    if num > n && counts.iter().all(|v| *v == 0) {
        return num;
    }

    let mut res = i32::MAX;
    for i in 1..10 {
        if counts[i] > 0 {
            counts[i] -= 1;
            let num = num * 10 + i as i32;
            res = res.min(backtracking2(counts, n, num));
            counts[i] += 1;
        }
    }

    res
}

fn next_beautiful_number(n: i32) -> i32 {
    let mut min = backtracking1(&mut [false; 10], n, 0);
    let mut counts = [0; 10];
    while min > 0 {
        counts[(min%10) as usize] += 1;
        min /= 10;
    }

    backtracking2(&mut counts, n, 0)
}

pub fn main() {
    let n = 1;
    println!("{}", next_beautiful_number(n));
}
