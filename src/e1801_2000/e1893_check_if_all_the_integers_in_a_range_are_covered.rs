fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut events = vec![];
    for range in ranges {
        events.push((range[0], 1));
        events.push((range[1]+1, -1));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut count = 0;
    let mut res = left;
    for event in events {
        if count == 0 && event.0 > res {
            break;
        }

        if count > 0 && event.1 == -1 {
            res = res.max(event.0);
        }

        count += event.1;
    }

    res > right
}

pub fn main() {
    let ranges = [[1,5],[8,14]].into_iter().map(Vec::from).collect();
    let left = 2;
    let right = 5;
    println!("{}", is_covered(ranges, left, right));
}
