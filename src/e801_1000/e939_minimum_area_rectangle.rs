use std::collections::HashSet;

fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    let mut hashset = HashSet::new();
    let n = points.len();
    for i in 0..n {
        hashset.insert((points[i][0], points[i][1]));
    }

    let mut res = i32::MAX;
    for i in 0..n {
        for j in i..n {
            let (x1, y1) = (points[i][0], points[i][1]);
            let (x2, y2) = (points[j][0], points[j][1]);
            if hashset.contains(&(x1, y2)) && hashset.contains(&(x2, y1)) {
                let val = (x2-x1) * (y2-y1);
                if val > 0 {
                    res = res.min(val);
                }
            }
        }
    }

    if res == i32::MAX { 0 } else { res }
}

pub fn main() {
    let points = [[1,1],[1,3],[3,1],[3,3],[2,2]].into_iter().map(Vec::from).collect();
    println!("{}", min_area_rect(points));
}
