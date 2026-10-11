fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    let (x3, y3) = (rec1[0].max(rec2[0]), rec1[1].max(rec2[1]));
    let (x4, y4) = (rec1[2].min(rec2[2]), rec1[3].min(rec2[3]));
    x4 > x3 && y4 > y3
}

pub fn main() {
    let rec1 = [0,0,2,2].to_vec();
    let rec2 = [1,1,3,3].to_vec();
    println!("{}", is_rectangle_overlap(rec1, rec2));
}
