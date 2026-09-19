fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
    properties.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

    let mut stack = vec![];
    let mut count = 0;
    let n = properties.len();
    for i in 0..n {
        while stack.last().is_some_and(|&k: &usize| properties[i][1] > properties[k][1] && properties[i][0] > properties[k][0]) {
            stack.pop();
            count += 1;
        }

        stack.push(i);
    }

    count
}

pub fn main() {
    let properties = [[1,5],[10,4],[4,3]].into_iter().map(Vec::from).collect();
    println!("{}", number_of_weak_characters(properties));
}
