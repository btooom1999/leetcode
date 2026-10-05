fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut prev = vec![];
    let mut res = i32::MAX;
    for num in nums {
        let mut cur = vec![];
        cur.push(num);

        for &prev_num in &prev {
            let next_num = prev_num | num;
            if cur.last().copied() != Some(next_num) {
                cur.push(next_num);
            }
        }

        for &num in &cur {
            res = res.min((num - k).abs());
        }

        if res == 0 {
            return 0;
        }

        prev = cur;
    }

    res
}

pub fn main() {
    let nums = [1,2,4,5].to_vec();
    let k = 3;
    println!("{}", minimum_difference(nums, k));
}
