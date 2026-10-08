fn maximum_beauty(flowers: Vec<i32>, mut new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
    let mut flowers = flowers.into_iter().map(|v| (v as i64, v as i64)).collect::<Vec<_>>();
    flowers.sort();

    let target = target as i64;
    let full = full as i64;
    let partial = partial as i64;
    let n = flowers.len();
    let mut r = n;
    for i in (0..n).rev() {
        if flowers[i].0 >= target {
            r -= 1;
        } else if new_flowers >= target - flowers[i].0 {
            new_flowers -= target - flowers[i].0;
            flowers[i].1 = target;
            r -= 1;
        } else {
            break;
        }
    }

    let mut res = (n-r) as i64 * full;
    if r == 0 {
        new_flowers += target - flowers[0].0;
        flowers[0].1 = flowers[0].0;
        r += 1;
    }

    let mut l = 0;
    let mut cur = flowers[0].1;
    let mut count = 0;
    while cur < target {
        while r < n && new_flowers < count {
            if flowers[r].0 >= target { break; }

            new_flowers += target - flowers[r].0;
            if flowers[r].0 < cur {
                new_flowers -= cur - flowers[r].0;
            }
            flowers[r].1 = flowers[r].0;
            r += 1;
        }

        if new_flowers < count {
            break;
        }

        new_flowers -= count;
        while l < n && cur >= flowers[l].1 {
            count += 1;
            l += 1;
        }
        res = res.max(cur * partial + (n-r) as i64 * full);
        cur += 1;
    }

    res
}

pub fn main() {
    // let flowers = [1,3,1,1].to_vec();
    // let new_flowers = 7;
    // let target = 6;
    // let full = 12;
    // let partial = 1;
    let flowers = [1,3].to_vec();
    let new_flowers = 7;
    let target = 10;
    let full = 1;
    let partial = 10;
    println!("{}", maximum_beauty(flowers, new_flowers, target, full, partial));
}
