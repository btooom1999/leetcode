fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heaps= vec![];
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            if i+j >= heaps.len() {
                heaps.push(vec![]);
            }

            heaps[i+j].push(nums[i][j]);
        }
    }

    heaps.into_iter().fold(Vec::new(), |mut acc, mut list| {
        while let Some(num) = list.pop() {
            acc.push(num);
        }

        acc
    })
}

pub fn main() {
    let nums = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_diagonal_order(nums));
}
