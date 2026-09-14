use std::collections::HashMap;

fn dfs(mut num: i32, hashmap: &HashMap<i32, String>) -> String {
    if num < 100 {
        if let Some(unit) = hashmap.get(&num) {
            return unit.to_string();
        }

        let num2 = num % 10;
        num = num / 10 * 10;
        return format!("{} {}", hashmap[&num], hashmap[&num2]);
    }

    let n = num.to_string().len();
    for i in 0..n {
        let base = (n-i-1) as u32;
        if let Some(unit) = hashmap.get(&10i32.pow(base)) {
            let num1 = num / 10_i32.pow(base);
            let num2 = num % 10_i32.pow(base);
            let first = dfs(num1, hashmap);
            if num2 != 0 {
                let second = dfs(num2, hashmap);
                return format!("{first} {unit} {second}");
            }

            return format!("{first} {unit}");
        }
    }

    unreachable!()
}

fn number_to_words(num: i32) -> String {
    let mut hashmap = std::collections::HashMap::new();
    hashmap.insert(0, "Zero".to_string());
    hashmap.insert(1, "One".to_string());
    hashmap.insert(2, "Two".to_string());
    hashmap.insert(3, "Three".to_string());
    hashmap.insert(4, "Four".to_string());
    hashmap.insert(5, "Five".to_string());
    hashmap.insert(6, "Six".to_string());
    hashmap.insert(7, "Seven".to_string());
    hashmap.insert(8, "Eight".to_string());
    hashmap.insert(9, "Nine".to_string());
    hashmap.insert(10, "Ten".to_string());
    hashmap.insert(11, "Eleven".to_string());
    hashmap.insert(12, "Twelve".to_string());
    hashmap.insert(13, "Thirteen".to_string());
    hashmap.insert(14, "Fourteen".to_string());
    hashmap.insert(15, "Fifteen".to_string());
    hashmap.insert(16, "Sixteen".to_string());
    hashmap.insert(17, "Seventeen".to_string());
    hashmap.insert(18, "Eighteen".to_string());
    hashmap.insert(19, "Nineteen".to_string());
    hashmap.insert(20, "Twenty".to_string());
    hashmap.insert(30, "Thirty".to_string());
    hashmap.insert(40, "Forty".to_string());
    hashmap.insert(50, "Fifty".to_string());
    hashmap.insert(60, "Sixty".to_string());
    hashmap.insert(70, "Seventy".to_string());
    hashmap.insert(80, "Eighty".to_string());
    hashmap.insert(90, "Ninety".to_string());
    hashmap.insert(100, "Hundred".to_string());
    hashmap.insert(1_000, "Thousand".to_string());
    hashmap.insert(1_000_000, "Million".to_string());
    hashmap.insert(1_000_000_000, "Billion".to_string());

    dfs(num, &hashmap)
}

pub fn main() {
    let num = 1001;
    println!("{}", number_to_words(num));
}
