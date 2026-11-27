fn count_points(rings: String) -> i32 {
    let mut flags = [[0, 0, 0]; 10];
    for window in rings.as_bytes().windows(2).step_by(2) {
        let i = (window[1] - b'0') as usize;
        let j = match window[0] {
            b'B' => 0,
            b'G' => 1,
            _ => 2,
        };

        flags[i][j] = 1;
    }

    flags.into_iter().map(|v| (v.into_iter().sum::<i32>() == 3) as i32).sum()
}

pub fn main() {
    let rings = "B0B6G0R6R0R6G9".to_string();
    println!("{}", count_points(rings));
}
