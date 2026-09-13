use std::collections::HashMap;

fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    for (i, &person) in row.iter().enumerate() {
        hashmap.insert(person, i);
    }

    let n = row.len();
    let mut res = 0;
    for i in (0..n).step_by(2) {
        if row[i] % 2 == 0 && row[i+1] != row[i]+1 {
            let j = row[i]+1;
            hashmap.insert(row[i+1], hashmap[&j]);
            row[hashmap[&j]] = row[i+1];
            res += 1;
        } else if row[i] % 2 != 0 && row[i+1] != row[i]-1 {
            let j = row[i]-1;
            hashmap.insert(row[i+1], hashmap[&j]);
            row[hashmap[&j]] = row[i+1];
            res += 1;
        }
    }

    res
}

pub fn main() {
    let row = [0,2,1,3].to_vec();
    println!("{}", min_swaps_couples(row));
}
