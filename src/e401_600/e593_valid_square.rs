use std::collections::HashSet;

fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();
    let points = [p1, p2, p3, p4];
    for i in 0..4 {
        for j in i+1..4 {
            let x = points[j][0]-points[i][0];
            let y = points[j][1]-points[i][1];
            hashset.insert(x*x+y*y);
        }
    }

    !hashset.contains(&0) && hashset.len() == 2
}

pub fn main() {
    let p1 = [0,0].to_vec();
    let p2 = [1,1].to_vec();
    let p3 = [1,0].to_vec();
    let p4 = [0,1].to_vec();
    println!("{}", valid_square(p1, p2, p3, p4));
}
