fn str_to_map(str: String) -> Vec<(usize, u8)> {
    let mut map = vec![];
    let str = str.as_bytes();
    let mut cur = str[0];
    let mut count = 0;
    for i in 0..str.len() {
        if cur == str[i] {
            count += 1;
        } else {
            map.push((count, cur));
            cur = str[i];
            count = 1;
        }
    }

    map.push((count, cur));
    map
}

fn map_to_str(map: Vec<(usize, u8)>) -> String {
    let mut str = String::new();
    for (count, byte) in map {
        str.push_str(&count.to_string());
        str.push(byte as char);
    }

    str
}

fn count_and_say(mut n: i32) -> String {
    let mut str = "1".to_string();
    n -= 1;
    while n > 0 {
        let map = str_to_map(str.clone());
        str = map_to_str(map);
        n -= 1;
    }

    str
}

pub fn main() {
    let n = 4;
    println!("{:?}", count_and_say(n));
}
