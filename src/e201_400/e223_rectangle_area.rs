fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
    let (ax3, ay3) = (ax1.max(bx1), ay1.max(by1));
    let (ax4, ay4) = (ax2.min(bx2), ay2.min(by2));

    (ax2-ax1)*(ay2-ay1)+(bx2-bx1)*(by2-by1)-(ax4-ax3).max(0)*(ay4-ay3).max(0)
}

pub fn main() {
    let ax1 = -3;
    let ay1 = 0;
    let ax2 = 3;
    let ay2 = 4;
    let bx1 = 0;
    let by1 = -1;
    let bx2 = 9;
    let by2 = 2;
    println!("{}", compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2));
}
