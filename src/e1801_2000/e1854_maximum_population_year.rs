fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut events = vec![];
    for log in &logs {
        events.push((log[0], 1));
        events.push((log[1], -1));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut max = 0;
    let mut cur_max = 0;
    let mut year = logs[0][0];
    for event in events {
        cur_max += event.1;
        if cur_max > max {
            max = cur_max;
            year = event.0;
        }
    }

    year
}

pub fn main() {
    let logs = [[1950,1961],[1960,1971],[1970,1981]].into_iter().map(Vec::from).collect();
    println!("{}", maximum_population(logs));
}
