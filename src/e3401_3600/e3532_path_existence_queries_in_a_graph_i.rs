fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }
}

fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut map = (0..n).collect::<Vec<_>>();

    for i in 1..n {
        if nums[i]-nums[i-1] <= max_diff {
            union(&mut map, i, i-1);
        }
    }

    let mut res = vec![];
    for query in queries {
        res.push(find(&mut map, query[0] as usize) == find(&mut map, query[1] as usize));
    }

    res
}

pub fn main() {
    let n = 2;
    let nums = [1,3].to_vec();
    let max_diff = 1;
    let queries = [[0,0],[0,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", path_existence_queries(n, nums, max_diff, queries))
}
