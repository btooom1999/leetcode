fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();

    let is_prime = |num: i32| -> bool {
        for i in 2..=num.isqrt() {
            if num % i == 0 { return false; }
        }

        true
    };

    let mut res = 0;
    for i in 0..n {
        if is_prime(nums[i][i]) {
            res = res.max(nums[i][i]);
        }

        if is_prime(nums[i][n-i-1]) {
            res = res.max(nums[i][n-i-1]);
        }
    }

    res
}

pub fn main() {
    let nums = [[1,2,3],[5,17,7],[9,11,10]].into_iter().map(Vec::from).collect();
    println!("{}", diagonal_prime(nums));
}
