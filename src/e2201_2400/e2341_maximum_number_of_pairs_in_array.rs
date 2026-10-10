fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut hashmap = [false; 101];
    let mut n = 0;
    let mut count = 0;
    for num in nums {
        let num = num as usize;
        if hashmap[num] {
            n -= 1;
            hashmap[num] = false;
            count += 1;
        } else {
            hashmap[num] = true;
            n += 1;
        }
    }

    vec![count, n]
}

pub fn main() {
    let nums = [1,3,2,1,3,2,2].to_vec();
    println!("{:?}", number_of_pairs(nums));
}
