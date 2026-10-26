fn divide_and_conquer(nums1: &[i32], nums2: &[i32], i: &mut usize, need: (usize, usize)) -> i32 {
    let (n1, n2) = (nums1.len(), nums2.len());
    if *i + n1 + n2 < need.0 {
        *i += n1 + n2;
        return 0;
    }

    if *i > need.1 {
        return 0;
    }

    if nums1.len() <= 1 && nums2.len() <= 1 {
        let mut sum = 0;
        if (n1 == 1 && n2 == 0) || (n2 == 1 && n1 == 0) {
            if need.0 == *i {
                sum += if n1 == 1 { nums1[0] } else { nums2[0] };
            }
            if need.1 == *i {
                sum += if n1 == 1 { nums1[0] } else { nums2[0] };
            }
            *i += 1;
        } else if n1 == 1 && n2 == 1 {
            let (min, max) = (nums1[0].min(nums2[0]), nums1[0].max(nums2[0]));
            if need.0 == *i {
                sum += min;
            }
            if need.1 == *i {
                sum += min;
            }
            *i += 1;
            if need.0 == *i {
                sum += max;
            }
            if need.1 == *i {
                sum += max;
            }
            *i += 1;
        }

        sum
    } else if n1 == 0 {
        divide_and_conquer(nums1, &nums2[..n2/2], i, need) + divide_and_conquer(nums1, &nums2[n2/2..], i, need)
    } else if n2 == 0 {
        divide_and_conquer(&nums1[..n1/2], nums2, i, need) + divide_and_conquer(&nums1[n1/2..], nums2, i, need)
    } else {
        let m = n2/2;
        let k = nums1.partition_point(|&v| v <= nums2[m]);
        if k == 0 && nums2[m] < nums1[k] {
            divide_and_conquer(&[], &nums2[..=m], i, need) + divide_and_conquer(nums1, &nums2[m+1..], i, need)
        } else {
            divide_and_conquer(&nums1[..k], &nums2[..m], i, need) + divide_and_conquer(&nums1[k..], &nums2[m..], i, need)
        }
    }
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len() + nums2.len();
    let need = if n.is_multiple_of(2) {
        (n/2-1, n/2)
    } else {
        (n/2, n/2)
    };

    divide_and_conquer(&nums1, &nums2, &mut 0, need) as f64 / 2f64
}

pub fn main() {
    let nums1 = [0,0,0,0].to_vec();
    let nums2 = [-1].to_vec();
    println!("{}", find_median_sorted_arrays(nums1, nums2));
}
