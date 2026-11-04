fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut l = 1;
    let mut r = arr[arr.len()-1]+k;

    while l < r {
        let m = (l+r)/2;
        let at = arr.partition_point(|&v| v <= m) as i32;

        if m - at < k {
            l = m+1;
        } else {
            r = m;
        }
    }

    l
}

pub fn main() {
    let arr = [1,2].to_vec();
    let k = 1;
    println!("{}", find_kth_positive(arr, k));
}
