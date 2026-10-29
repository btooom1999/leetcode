fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    let mut events = vec![];
    for interval in intervals {
        events.push((interval[0], 1));
        events.push((interval[1], -1));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut count = 0;
    for event in events {
        count += event.1;
        if count > 1 {  return false; }
    }

    true
}

pub fn main() {
    let intervals = [[0,30],[5,10],[30,31]].into_iter().map(Vec::from).collect();
    println!("{:?}", can_attend_meetings(intervals));
}
