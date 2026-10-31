fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    let mut people = people.into_iter().enumerate().collect::<Vec<_>>();
    people.sort_by_key(|a| a.1);

    let mut events = vec![];
    for flower in flowers {
        events.push((flower[0], 1));
        events.push((flower[1]+1, -1));
    }

    events.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let n = people.len();
    let mut res = vec![0; n];
    let mut i = 0;
    let mut count = 0;
    for event in events {
        while i < n && people[i].1 < event.0 {
            res[people[i].0] = count;
            i += 1;
        }
        count += event.1;
    }

    res
}

pub fn main() {
    let flowers = [[1,6],[3,7],[9,12],[4,13]].into_iter().map(Vec::from).collect();
    let people = [2,3,7,11].to_vec();
    println!("{:?}", full_bloom_flowers(flowers, people));
}
