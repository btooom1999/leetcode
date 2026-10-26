fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    todo!()
}

pub fn main() {
    let points = [[2,2],[1,2],[1,0],[2,0]].into_iter().map(Vec::from).collect();
    println!("{}", min_area_free_rect(points));
}
