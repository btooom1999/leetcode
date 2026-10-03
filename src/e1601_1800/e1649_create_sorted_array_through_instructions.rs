const MOD: i64 = 1_000_000_007;

fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    let max = *instructions.iter().max().unwrap() as usize + 1;
    let mut tree = (0..=max).map(|v| (v, 0)).collect::<Vec<_>>();

    let mut res = 0;
    let n = instructions.len();
    let mut suffix_count = 0;
    for i in 0..n {
        suffix_count += 1;
        let mut k = instructions[i] as usize;
        while k < max {
            tree[k].1 += 1;
            k += k & !(k-1);
        }
        let mut k = tree.partition_point(|v| v.0 < instructions[i] as usize)-1;
        let mut prefix_count = 0;
        while k > 0 {
            prefix_count += tree[k].1;
            k -= k & !(k-1);
        }

        let mut suffix_excess_count = 0;
        let mut k = tree.partition_point(|v| v.0 <= instructions[i] as usize)-1;
        while k > 0 {
            suffix_excess_count += tree[k].1;
            k -= k & !(k-1);
        }

        res = (res + prefix_count.min(suffix_count-suffix_excess_count)) % MOD;
    }

    res as i32
}

pub fn main() {
    let instructions = [4,14,10,2,5,3,8,19,7,20,12,1,9,15,13,11,18,6,16,17].to_vec();
    println!("{}", create_sorted_array(instructions));
}
