fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
    let mut max = weight[0];
    let mut count = 0;
    let n = weight.len();
    for i in 0..n {
        if weight[i] >= max {
            max = weight[i];
        } else {
            count += 1;
            max = *weight.get(i+1).unwrap_or(&0);
        }
    }

    count
}

pub fn main() {
    let weight = [2,5,1,4,3].to_vec();
    println!("{}", max_balanced_shipments(weight));
}
