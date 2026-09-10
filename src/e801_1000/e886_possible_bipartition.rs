fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let n = n as usize;

    let mut map = vec![vec![]; n+1];
    for dislike in dislikes {
        map[dislike[0] as usize].push(dislike[1] as usize);
        map[dislike[1] as usize].push(dislike[0] as usize);
    }

    let mut colors = vec![-1; n+1];

    for i in 1..=n {
        if colors[i] == -1 {
            colors[i] = 1;
            let mut q = std::collections::VecDeque::from([(i, false)]);
            while let Some((a, color)) = q.pop_front() {
                for &b in &map[a] {
                    if colors[a] == colors[b] { return false; }
                    if colors[b] == -1 {
                        colors[b] = color as i32;
                        q.push_back((b, !color));
                    }
                }
            }
        }
    }

    true
}

pub fn main() {
    let n = 3;
    let dislikes = vec![vec![1,2],vec![1,3],vec![2,3]];
    println!("{}", possible_bipartition(n, dislikes));
}
