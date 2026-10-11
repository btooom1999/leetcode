fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let n = points.len();
    let mut res = 0f64;
    for x in 0..n {
        for y in x+1..n {
            for z in y+1..n {
                let (x1, y1) = (points[x][0], points[x][1]);
                let (x2, y2) = (points[y][0], points[y][1]);
                let (x3, y3) = (points[z][0], points[z][1]);
                res = res.max(0.5f64 * ((x1*y2+x2*y3+x3*y1) as f64 - (y1*x2+y2*x3+y3*x1) as f64).abs())
            }
        }
    }

    res
}

pub fn main() {
    let points = [[0,0],[0,1],[1,0],[0,2],[2,0]].into_iter().map(Vec::from).collect();
    println!("{}", largest_triangle_area(points));
}
