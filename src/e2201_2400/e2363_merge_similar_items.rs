use std::collections::HashMap;

fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hashmap = HashMap::new();
    for item in items1 {
        hashmap.entry(item[0]).or_insert(item[1]);
    }
    for item in items2 {
        hashmap.entry(item[0]).and_modify(|v| *v += item[1]).or_insert(item[1]);
    }

    let mut res = hashmap.into_iter().map(|v| vec![v.0, v.1]).collect::<Vec<_>>();
    res.sort_unstable();
    res
}

pub fn main() {
    let items1 = [[1,1],[4,5],[3,8]].into_iter().map(Vec::from).collect();
    let items2 = [[3,1],[1,5]].into_iter().map(Vec::from).collect();
    println!("{:?}", merge_similar_items(items1, items2));
}
