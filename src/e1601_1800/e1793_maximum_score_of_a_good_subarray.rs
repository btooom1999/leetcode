fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut left = vec![];
    let mut right = vec![];
    for i in (0..=k).rev() {
        if let Some(&j) = left.last() && nums[j] <= nums[i] {
            left.push(j);
        } else {
            left.push(i);
        }
    }

    let n = nums.len();
    for i in k..n {
        if let Some(&j) = right.last() && nums[j] <= nums[i] {
            right.push(j);
        } else {
            right.push(i);
        }
    }

    let mut l = 0;
    let mut r = n-1;
    let mut res = nums[k];
    let mut l_val = nums[left.pop().unwrap()];
    let mut r_val = nums[right.pop().unwrap()];
    while !left.is_empty() || !right.is_empty() {
        res = res.max(l_val.min(r_val) * (r-l+1) as i32);
        if left.is_empty() {
            r_val = nums[right.pop().unwrap()];
            r -= 1;
        } else if right.is_empty() || l_val < r_val {
            l_val = nums[left.pop().unwrap()];
            l += 1;
        } else {
            r_val = nums[right.pop().unwrap()];
            r -= 1;
        }
    }

    res
}

pub fn main() {
    let nums = [1,4,3,7,4,5].to_vec();
    let k = 3;
    println!("{}", maximum_score(nums, k));
}
