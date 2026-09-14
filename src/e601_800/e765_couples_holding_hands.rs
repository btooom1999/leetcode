fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
let n = row.len();
    let mut hashmap = vec![0; n];
    for (i, &person) in row.iter().enumerate() {
        hashmap[person as usize] = i;
    }

    let mut res = 0;
    for i in (0..n).step_by(2) {
        if row[i] % 2 == 0 && row[i+1] != row[i]+1 {
            let j = (row[i]+1) as usize;
            hashmap[row[i+1] as usize] = hashmap[j];
            row[hashmap[j]] = row[i+1];
            res += 1;
        } else if row[i] % 2 != 0 && row[i+1] != row[i]-1 {
            let j = (row[i]-1) as usize;
            hashmap[row[i+1] as usize] = hashmap[j];
            row[hashmap[j]] = row[i+1];
            res += 1;
        }
    }

    res
}

pub fn main() {
    let row = [0,2,1,3].to_vec();
    println!("{}", min_swaps_couples(row));
}
