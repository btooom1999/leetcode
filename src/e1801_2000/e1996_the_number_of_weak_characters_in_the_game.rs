fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
    properties.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

    let mut count = 0;
    let n = properties.len();
    let mut cur = (0, 0);
    for i in (0..n).rev() {
        if properties[i][1] > cur.1 {
            cur = (properties[i][0], properties[i][1]);
        } else if properties[i][1] < cur.1 {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let properties = [[1,5],[10,4],[4,3]].into_iter().map(Vec::from).collect();
    println!("{}", number_of_weak_characters(properties));
}
