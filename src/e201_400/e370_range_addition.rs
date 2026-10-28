fn get_modiied_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
    let length = length as usize;
    let mut diff = vec![0; length+1];
    for update in updates {
        let (a, b, delta) = (update[0] as usize, update[1] as usize, update[2]);
        diff[a] += delta;
        diff[b+1] -= delta;
    }

    for i in 1..length+1 {
        diff[i] += diff[i-1];
    }

    diff.pop();
    diff
}

pub fn main() {
    let length = 5;
    let updates = [[1, 3, 2], [2, 4, 3]].into_iter().map(Vec::from).collect();
    println!("{:?}", get_modiied_array(length, updates));
}
