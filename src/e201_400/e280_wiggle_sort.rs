fn wiggle_sort(nums: &mut [i32]) {
    if nums.len() == 1 {
        return;
    }

    let mut hashmap = vec![0; 5_001];
    for num in nums.iter() {
        hashmap[*num as usize] += 1;
    }

    let mut hashmap = hashmap.into_iter().enumerate().filter_map(|(i, count)| {
        if count == 0 {
            None
        } else {
            Some((i as i32, count)) // (number, count)
        }
    }).collect::<Vec<_>>();

    let mut i = 0;
    let mut j = hashmap.len() - 1 ;
    let mut x = 0;

    loop {
        if hashmap[i].1 == 0 || hashmap[j].1 == 0 {
            break;
        }

        if x < nums.len() {
            nums[x] = hashmap[i].0;
            hashmap[i].1 -= 1;
            if hashmap[i].1 == 0 && i < hashmap.len() - 1 {
                i += 1;
            }
        }

        if x + 1 < nums.len() && hashmap[j].1 > 0 {
            nums[x+1] = hashmap[j].0;
            hashmap[j].1 -= 1;
            if hashmap[j].1 == 0 && j > 0 {
                j -= 1;
            }
        }

        x += 2;
    }
}

pub fn main() {
    let mut nums = vec![3, 5, 2, 1, 6, 4];
    wiggle_sort(&mut nums);
    println!("{:?}", nums);
}

