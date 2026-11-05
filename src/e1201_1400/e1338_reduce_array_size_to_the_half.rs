fn min_set_size(arr: Vec<i32>) -> i32 {
    let mut hashmap = std::collections::HashMap::<_, usize>::new();

    let n = arr.len();
    for num in arr {
        *hashmap.entry(num).or_default() += 1;
    }

    let mut hashmap = hashmap.into_iter().collect::<Vec<_>>();
    hashmap.sort_by(|a, b| b.1.cmp(&a.1));

    let mut amount = n;
    let mut i = 0;
    while amount > n/2 {
        amount -= hashmap[i].1;
        i += 1;
    }

    i as i32
}

pub fn main() {
    let arr = [3,3,3,3,5,5,5,2,2,7].to_vec();
    println!("{}", min_set_size(arr));
}
