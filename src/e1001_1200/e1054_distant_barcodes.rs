fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut hashmap = std::collections::HashMap::<_, i32>::new();

    for num in barcodes {
        *hashmap.entry(num).or_default() += 1
    }

    let mut hashmap = hashmap.into_iter().map(|v| (v.1, v.0)).collect::<std::collections::BinaryHeap::<_>>();

    let mut res = vec![];
    let mut removed = vec![];

    while !hashmap.is_empty() {
        if let Some((c, n)) = hashmap.pop() {
            if c != 1 {
                removed.push((c-1, n));
            }
            res.push(n);
        }
        if let Some((c, n)) = hashmap.pop() {
            if c != 1 {
                removed.push((c-1, n));
            }
            res.push(n);
        }

        while let Some(item) = removed.pop() {
            hashmap.push(item);
        }
    }

    res
}

pub fn main() {
    let barcodes = [1,1,1,2,2,2].to_vec();
    println!("{:?}", rearrange_barcodes(barcodes));
}
