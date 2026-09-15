fn compare_version(version1: String, version2: String) -> i32 {
    let mut a = version1.split(".").collect::<Vec<_>>();
    let mut b = version2.split(".").collect::<Vec<_>>();
    let n = a.len().max(b.len());
    while a.len() < n {
        a.push("0");
    }
    while b.len() < n {
        b.push("0");
    }

    for (a, b) in a.into_iter().zip(b.into_iter()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }

    0
}

pub fn main() {
    let version1 = "1.2".to_string();
    let version2 = "1.10.0".to_string();
    println!("{}", compare_version(version1, version2));
}
