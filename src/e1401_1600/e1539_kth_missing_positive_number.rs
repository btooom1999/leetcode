fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut l = 1;
    let mut r = arr[arr.len()-1]+k;

    while l < r {
        let m = (l+r)/2;
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = (left+right)/2;
            if arr[mid] <= m {
                left = mid+1;
            } else {
                right = mid;
            }
        }

        if m - (left as i32) < k {
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
