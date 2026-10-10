fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
    points.sort_unstable();

    let mut res = 1;
    let mut distance = points[0][0] + w;
    for point in points {
        if point[0] > distance {
            res += 1;
            distance = point[0]+w;
        }
    }
    res
}

pub fn main() {
    let points = [[2,1],[1,0],[1,4],[1,8],[3,5],[4,6]].into_iter().map(Vec::from).collect();
    let w = 1;
    println!("{}", min_rectangles_to_cover_points(points, w));
}
