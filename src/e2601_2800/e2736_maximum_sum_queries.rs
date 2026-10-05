fn update(tree: &mut [i32], mut k: usize, val: i32) {
    while k < tree.len() {
        tree[k] = tree[k].max(val);
        k += k & k.wrapping_neg();
    }
}

fn query(tree: &mut [i32], mut k: usize) -> i32 {
    let mut res = -1;
    while k > 0 {
        res = res.max(tree[k]);
        k -= k & k.wrapping_neg();
    }

    res
}

pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums1.len();
    let mut pairs = (0..n).map(|i| (nums1[i], nums2[i])).collect::<Vec<_>>();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));

    let mut coords = (0..n).map(|i| nums2[i]).collect::<Vec<_>>();
    coords.sort_unstable();
    coords.dedup();

    let mut res = vec![-1; queries.len()];
    let mut q = queries.into_iter().enumerate().map(|v| (v.1[0], v.1[1], v.0)).collect::<Vec<_>>();
    q.sort_by(|a, b| b.0.cmp(&a.0));

    let n = pairs.len();
    let m = coords.len();
    let mut tree = vec![-1; m+1];
    let mut qi = 0;
    for (x, y, i) in q {
        while qi < n && pairs[qi].0 >= x {
            let pos = coords.binary_search(&pairs[qi].1).unwrap();
            update(&mut tree, m-pos, pairs[qi].0 + pairs[qi].1);
            qi += 1;
        }

        let pos = coords.partition_point(|&v| v < y);
        res[i] = query(&mut tree, m-pos);
    }

    res
}

pub fn main() {
    let nums1 = [4,3,1,2].to_vec();
    let nums2 = [2,4,9,5].to_vec();
    let queries = [[4,5],[1,3],[2,5]].into_iter().map(Vec::from).collect();
    // let nums1 = [66,69].to_vec();
    // let nums2 = [39,19].to_vec();
    // let queries = [[4,63]].into_iter().map(Vec::from).collect();
    println!("{:?}", maximum_sum_queries(nums1, nums2, queries));
}
