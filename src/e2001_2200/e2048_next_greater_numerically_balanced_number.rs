fn next_beautiful_number(n: i32) -> i32 {
    let is_beautiful_number = |freq: &[usize; 10]| -> bool {
        for i in 0..10 {
            if freq[i] != 0 && i != freq[i] {
                return false;
            }
        }

        true
    };

    let mut freq = [0; 10];
    for num in n+1..1224445 {
        let mut x = num;
        while x > 0 {
            freq[(x % 10) as usize] += 1;
            x /= 10;
        }

        if is_beautiful_number(&freq) {
            return num;
        }

        freq = [0; 10];
    }

    unreachable!()
}

pub fn main() {
    let n = 1;
    println!("{}", next_beautiful_number(n));
}
