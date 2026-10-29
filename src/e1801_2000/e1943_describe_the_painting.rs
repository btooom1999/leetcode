fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
    let mut events = vec![];
    for seg in segments {
        events.push((seg[0], 1, seg[2] as i64));
        events.push((seg[1], -1, seg[2] as i64));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut color = events[0].2;
    let mut res = vec![];
    for i in 1..events.len() {
        if color > 0 && events[i].0 != events[i-1].0 {
            res.push(vec![events[i-1].0 as i64, events[i].0 as i64, color]);
        }
        if events[i].1 == 1 {
            color += events[i].2;
        } else {
            color -= events[i].2;
        }
    }

    res
}

pub fn main() {
    let segments = [[1,4,5],[4,7,7],[1,7,9]].into_iter().map(Vec::from).collect();
    println!("{:?}", split_painting(segments));
}
