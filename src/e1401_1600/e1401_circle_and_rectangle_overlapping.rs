fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let x_closest = if x_center < x1 {
        x1
    } else if x_center > x2 {
        x2
    } else {
        x_center
    };

    let y_closest = if y_center < y1 {
        y1
    } else if y_center > y2 {
        y2
    } else {
        y_center
    };

    let a = x_closest - x_center;
    let b = y_closest - y_center;
    a*a + b*b <= radius*radius
}

pub fn main() {
    let radius = 5;
    let x_center = 7;
    let y_center = 2;
    let x1 = 12;
    let y1 = 0;
    let x2 = 19;
    let y2 = 3;
    println!("{}", check_overlap(radius, x_center, y_center, x1, y1, x2, y2));
}
