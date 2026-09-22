fn count_groups(position: Vec<i32>, speed: Vec<i32>, distance: i32) -> i32 {
    let n = position.len();
    let mut data = vec![];
    for i in 0..n {
        if i+1 < n && position[i+1]-position[i]<=distance {
            continue;
        }

        data.push(i);
    }

    let mut stack = vec![];
    for i in (0..data.len()).rev() {
        if stack.last().is_none_or(|&k| speed[data[k]] >= speed[data[i]]) {
            stack.push(i);
        }
    }

    stack.len() as i32
}

pub fn main() {
    let position = [589,843,847].to_vec();
    let speed = [452,590,834].to_vec();
    let distance = 254;
    // let position = [677,711,942,960].to_vec();
    // let speed = [774,951,743,516].to_vec();
    // let distance = 27;
    // let position = [1,5,6,20].to_vec();
    // let speed = [4,3,2,3].to_vec();
    // let distance = 1;
    println!("{}", count_groups(position, speed, distance));
}
