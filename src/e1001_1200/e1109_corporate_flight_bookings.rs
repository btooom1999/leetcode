fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut diff = vec![0; n+1];
    for booking in bookings {
        diff[booking[0] as usize - 1] += booking[2];
        diff[booking[1] as usize] -= booking[2];
    }

    for i in 1..n {
        diff[i] += diff[i-1];
    }

    diff.pop();
    diff
}

pub fn main() {
    let bookings = [[1,2,10],[2,3,20],[2,5,25]].into_iter().map(Vec::from).collect();
    let n = 5;
    println!("{:?}", corp_flight_bookings(bookings, n));
}
