fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut res = target[0];
    let n = target.len();
    for i in 1..n {
        if target[i] > target[i-1] {
            res += target[i] - target[i-1];
        }
    }

    res
}

pub fn main() {
    let target = [1,2,3,2,1].to_vec();
    println!("{}", min_number_operations(target));
}
