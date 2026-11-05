fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut l = 0;
    let mut r = nums.iter().sum::<i32>();

    while l < r {
        let m = (l+r)/2;

        let mut groups = 1;
        let mut sum = 0;
        for i in 0..n {
            if sum + nums[i] > m {
                groups += 1;
                sum = 0;
            }

            if sum + nums[i] > m {
                groups = k+1;
                break;
            }

            sum += nums[i];
        }

        if groups > k {
            l = m+1;
        } else {
            r = m;
        }
    }

    l
}

pub fn main() {
    let nums = [7,2,5,10,8].to_vec();
    let k = 2;
    println!("{}", split_array(nums, k));
}
