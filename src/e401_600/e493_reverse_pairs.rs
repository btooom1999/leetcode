use std::collections::BTreeMap;

fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut btreemap = BTreeMap::<_, i32>::new();
    let mut total = 0;
    for &num in &nums {
        total += 1;
        *btreemap.entry(num).or_default() += 1;
    }

    let data = btreemap.into_iter().collect::<Vec<_>>();
    let n = data.len();
    let mut tree = vec![0; n+1];
    for i in 0..n {
        let mut k = i+1;
        while k <= n {
            tree[k] += data[i].1;
            k += k & !(k-1);
        }
    }

    let mut res = 0;
    for &num in nums.iter().rev() {
        total -= 1;
        let mut k = data.partition_point(|v| v.0 <= num);
        while k <= n {
            tree[k] -= 1;
            k += k & !(k-1);
        }

        let mut k = data.partition_point(|v| v.0 as i64 <= 2 * num as i64);
        let mut count = 0;
        while k > 0 {
            count += tree[k];
            k -= k & !(k-1);
        }

        res += total - count;
    }

    res
}

pub fn main() {
    let nums = [2147483647,2147483647,2147483647,2147483647,2147483647,2147483647].to_vec();
    println!("{}", reverse_pairs(nums));
}
