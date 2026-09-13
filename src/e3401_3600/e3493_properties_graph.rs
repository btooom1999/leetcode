fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) -> usize {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }

    root_a.min(root_b)
}

fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = properties.len();
    let mut map = (0..n).collect::<Vec<_>>();

    let mut res = [false; 101];
    for a in 0..n {
        let mut hashset = [false; 101];
        for &num in &properties[a] {
            hashset[num as usize] = true;
        }

        for b in a+1..n {
            let mut count = 0;
            let mut hashset = hashset.to_vec();
            for &num in &properties[b] {
                count += hashset[num as usize] as i32;
                hashset[num as usize] = false;
            }

            if count >= k {
                union(&mut map, a, b);
            }
        }
    }

    for i in 0..n {
        res[find(&mut map, i)] = true;
    }

    res.iter().fold(0, |acc, &flag| acc + flag as i32)
}

pub fn main() {
    let properties = [[1,2],[1,1],[3,4],[4,5],[5,6],[7,7]].into_iter().map(Vec::from).collect();
    let k = 1;
    println!("{}", number_of_components(properties, k));
}
