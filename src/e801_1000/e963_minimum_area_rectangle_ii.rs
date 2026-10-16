use std::collections::HashSet;

fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    let mut res = f64::MAX;
    let n = points.len();
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                for d in 0..n {
                    let (x1, y1) = (points[a][0], points[a][1]);
                    let (x2, y2) = (points[b][0], points[b][1]);
                    let (x3, y3) = (points[c][0], points[c][1]);
                    let (x4, y4) = (points[d][0], points[d][1]);

                    let hashset = HashSet::from([
                        (x2-x1)*(x2-x1)+(y2-y1)*(y2-y1),
                        (x3-x1)*(x3-x1)+(y3-y1)*(y3-y1),
                        (x4-x1)*(x4-x1)+(y4-y1)*(y4-y1),
                        (x3-x2)*(x3-x2)+(y3-y2)*(y3-y2),
                        (x4-x2)*(x4-x2)+(y4-y2)*(y4-y2),
                        (x4-x3)*(x4-x3)+(y4-y3)*(y4-y3),
                    ]);

                    if !hashset.contains(&0) && (hashset.len() == 2 || hashset.len() == 3) {
                        let min = ((x1*y2+x2*y3+x3*y4+x4*y1)-(y1*x2+y2*x3+y3*x4+y4*x1)).abs() as f64 / 2f64;
                    }
                }
            }
        }
    }

    if res == f64::MAX { 0f64 } else { res }
}

// [[0, 1], [1, 0], [1, 2], [2, 1]]
// 0x0 + 1x2 + 1x1 + 2x1 = 5
// 1x1 + 0x1 + 2x2 + 1x0 = 5

pub fn main() {
    let points = [[2,2],[1,2],[1,0],[2,0]].into_iter().map(Vec::from).collect();
    println!("{}", min_area_free_rect(points));
}
