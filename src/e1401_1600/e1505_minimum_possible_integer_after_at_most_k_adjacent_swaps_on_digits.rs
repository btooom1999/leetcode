fn add(tree: &mut [(bool, usize)], mut k: usize) {
    let n = tree.len();
    while k < n {
        tree[k].1 += 1;
        k += k & k.wrapping_neg();
    }
}

fn query(tree: &[(bool, usize)], mut k: usize) -> usize {
    let mut jump = 0;
    while k > 0 {
        jump += tree[k].1;
        k -= k & k.wrapping_neg();
    }

    jump
}

fn min_integer(num: String, k: i32) -> String {
    let n = num.len();
    let bytes= num.into_bytes();
    let mut hashmap = vec![vec![]; 10];
    let mut tree = vec![(false, 0); n+1];

    for i in (0..n).rev() {
        hashmap[(bytes[i] - b'0') as usize].push(i);
    }

    let mut k = k as usize;
    let mut res = vec![];
    let mut i = 0;
    while res.len() < bytes.len() {
        if tree[i+1].0 {
            i += 1;
            continue;
        }

        let mut b = bytes[i];
        let jump_i = query(&tree, i+1);
        'outer: for num in 0..(bytes[i]-b'0') as usize {
            while hashmap[num].last().is_some_and(|&j| j < i) {
                hashmap[num].pop();
            }

            if let Some(&j) = hashmap[num].last() {
                let jump_j = query(&tree, j+1);
                println!("{} {} {} {}", jump_j, jump_i, j, i);
                if j-i-(jump_j-jump_i) <= k {
                    b = bytes[j];
                    k -= j-i-(jump_j-jump_i);
                    tree[j+1].0 = true;
                    add(&mut tree, j+1);
                    hashmap[num].pop();
                    break 'outer;
                }
            }
        }
        if b == bytes[i] {
            i += 1;
        }

        res.push(b);
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let num = "4321".to_string();
    let k = 4;
    println!("{}", min_integer(num, k));
}
