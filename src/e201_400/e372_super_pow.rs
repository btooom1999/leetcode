fn super_pow(a: i32, b: Vec<i32>) -> i32 {
    let a = a as i128 % 1337;
    let mut res = 1i128;
    for base in b {
        res = res.pow(10) % 1337;
        res = (res * a.pow(base as u32) % 1337) % 1337;
    }

    res as i32
}

pub fn main() {
    let a = 2;
    let b = [4,3].to_vec();
    println!("{}", super_pow(a, b));
}
