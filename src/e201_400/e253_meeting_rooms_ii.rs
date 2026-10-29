#[derive(Debug, Clone)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(time: (i32, i32)) -> Self {
        Interval { start: time.0, end: time.1 }
    }
}

fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
    let mut events = vec![];
    for interval in intervals {
        events.push((interval.start, 1));
        events.push((interval.end, -1));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut res = 0;
    let mut count = 0;

    for event in events {
        count += event.1;
        res = res.max(count);
    }

    res
}

pub fn main() {
    let intervals = [(0,40),(5,10),(15,20)].into_iter().map(Interval::new).collect();
    println!("{:?}", min_meeting_rooms(intervals));
}
