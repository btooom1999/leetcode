fn build(nums: &[(i32, i32, usize, i32)]) -> (Vec<(i32, usize, i32)>, usize) {
    let n = nums.len();
    let mut initial_n = 1;
    while initial_n < n {
        initial_n <<= 1;
    }

    let mut tree = vec![(0,0,0); 2*initial_n-1];
    let bias = initial_n-1;
    for i in 0..n {
        tree[i+bias] = (nums[i].0, nums[i].2, nums[i].3);
    }
    for i in (0..bias).rev() {
        let a = tree[2*i+1];
        let b = tree[2*i+2];
        if a.2 > b.2 {
            tree[i] = a;
        } else  {
            tree[i] = b;
        }
    }

    (tree, initial_n)
}

fn query(
    tree: &[(i32, usize, i32)],
    nums1: &[i32],
    nums2: &[i32],
    l: usize,
    r: usize,
    q_l: usize,
    q_r: usize,
    pos: usize,
    current: (i32, i32),
) -> i32 {
    if r < q_l || q_r < l {
        return -1;
    }

    if q_l <= l && r <= q_r {
        if nums1[tree[pos].1] >= current.0 && nums2[tree[pos].1] >= current.1 {
            return tree[pos].2;
        } else if l == r {
            return -1;
        }
    }

    let m = (l+r)/2;
    query(tree, nums1, nums2, l, m, q_l, q_r, 2*pos+1, current).max(query(tree, nums1, nums2, m+1, r, q_l, q_r, 2*pos+2, current))
}

fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums1.len();
    let mut data = vec![];
    for i in 0..n {
        if nums1[i] <= nums2[i] {
            data.push((nums1[i], 0, i, nums1[i] + nums2[i]));
        } else if nums1[i] > nums2[i] {
            data.push((nums2[i], 1, i, nums1[i] + nums2[i]));
        }
    }

    data.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mid = data.partition_point(|v| v.1 == 0);
    let (above_nums, below_nums) = data.split_at(mid);
    let (above_tree, above_len) = build(above_nums);
    let (below_tree, below_len) = build(below_nums);

    let mut res = vec![-1; queries.len()];
    for (i, data) in queries.iter().enumerate() {
        let n = above_nums.len();
        if n>0 {
            let (q_l, q_r) = (above_nums.partition_point(|v| v.0 < data[0]), n-1);
            let val = query(&above_tree, &nums1, &nums2, 0, above_len-1, q_l, q_r, 0, (data[0], data[1]));
            res[i] = val;
        }

        let n = below_nums.len();
        if n>0 {
            let (q_l, q_r) = (below_nums.partition_point(|v| v.0 < data[1]), n-1);
            let val = query(&below_tree, &nums1, &nums2, 0, below_len-1, q_l, q_r, 0, (data[0], data[1]));
            res[i] = res[i].max(val);
        }
    }

    res
}

pub fn main() {
    let nums1 = [3,2,5,3,4,2,5,3,4].to_vec();
    let nums2 = [3,2,5,3,4,2,5,3,4].to_vec();
    let queries = [[2,2],[3,4],[5,5]].into_iter().map(Vec::from).collect();
    // let nums1 = [66,69].to_vec();
    // let nums2 = [39,19].to_vec();
    // let queries = [[4,63]].into_iter().map(Vec::from).collect();
    println!("{:?}", maximum_sum_queries(nums1, nums2, queries));
}
