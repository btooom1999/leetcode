fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut counts = [0; 201];
    for num in nums {
        counts[num as usize] += 1;
    }

    let mut res = vec![];
    while counts.iter().any(|&v| v > 0) {
        let mut inner = vec![];
        for i in 1..201 {
            if counts[i] > 0 {
                inner.push(i as i32);
                counts[i] -= 1;
            }
        }

        res.push(inner);
    }

    res
}

pub fn main() {
    let nums = [1,3,4,1,2,3,1].to_vec();
    println!("{:?}", find_matrix(nums));
}
